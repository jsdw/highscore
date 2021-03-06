//! A convenient response type to use in our API handlers. Any errors that come
//! from things implementing the `Store` interface can be cast to these.

use std::io::Cursor;
use crate::store_interface::{ StoreError };
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ Status, ContentType };

/// An HttpResult returns either error that anything implementing
/// HasErrorKind can be converted into, or a response.
pub type HttpResult<T> = Result<T,HttpError>;

pub struct HttpError {
    pub code: u16,
    pub message: String
}

impl HttpError {
    pub fn new<S: Into<String>>(code: u16, message: S) -> HttpError {
        HttpError { code, message: message.into() }
    }
    pub fn server_error<S: Into<String>>(message: S) -> HttpError {
        HttpError { code: 500, message: message.into() }
    }
}

// Anything that is a valid `store_interface` Error can also
// be converted into an HttpError:
impl From<StoreError> for HttpError {
    fn from(e: StoreError) -> Self {
        HttpError {
            code: 400,
            message: e.to_string()
        }
    }
}

// With this, Rocket understands how to respond to the user given an HttpError:
#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for HttpError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let status = Status::from_code(self.code).unwrap_or(Status::InternalServerError);
        Response::build()
            .header(ContentType::Plain)
            .status(status)
            .sized_body(self.message.len(), Cursor::new(self.message))
            .ok()
    }
}
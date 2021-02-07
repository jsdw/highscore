use std::fmt::Display;
use std::io::Cursor;
use crate::store_interface::{ ErrorKind, HasErrorKind };
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
}

impl <E: HasErrorKind + Display> From<E> for HttpError {
    fn from(e: E) -> Self {
        let status_code = match e.error_kind() {
            ErrorKind::UserError => 400
        };
        HttpError {
            code: status_code,
            message: e.to_string()
        }
    }
}

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
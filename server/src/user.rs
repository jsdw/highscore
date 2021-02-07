use std::convert::TryInto;

use rocket::request::{self, Request, FromRequest};
use rocket::http::{CookieJar, Cookie, Status};

static COOKIE_NAME: &str = "username";

/// This can be obtained via a FromRequest, and is
/// only handed back if a valid user is present.
pub struct User {
    pub name: String
}

// This allows the thing to be asked for in a rocket request.
// If the user is not loogged in, it'll return an Unauthorized status.
#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    async fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let username = req.cookies()
            .get_private(COOKIE_NAME)
            .map(|c| c.value().to_owned());

        match username {
            Some(name) => request::Outcome::Success(User { name }),
            None => request::Outcome::Failure((Status::Unauthorized,()))
        }
    }
}

/// Set a cookie for a user so that they are logged in
pub fn add_user_cookie(cookies: &CookieJar, username: String) {
    cookies.add_private(build_cookie(username));
}

/// Log a user out by removing their cookie
pub fn remove_user_cookie(cookies: &CookieJar) {
    cookies.remove_private(build_cookie(String::new()));
}

fn build_cookie(username: String) -> Cookie<'static> {
    let expires = time::OffsetDateTime::now_utc() + time::Duration::days(60);
    Cookie::build(COOKIE_NAME, username)
        .expires(expires)
        .path("/")
        .http_only(true)
        .finish()
}
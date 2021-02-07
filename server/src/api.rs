use crate::store_interface::{ HashedPassword, Store };
use crate::persisted_store::{ PersistedStore };
use crate::user::{ self, User };
use crate::http_result::{ HttpError, HttpResult };
use serde::{ Serialize, Deserialize };
use rocket_contrib::json::Json;
use rocket::{State, http::CookieJar};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        logout,
        upsert_user
    ]
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String
}

#[post("/api/login", data = "<body>")]
async fn login(store: State<'_, PersistedStore>, cookies: &CookieJar<'_>, body: Json<LoginInput>) -> HttpResult<Json<()>> {
    let is_valid = store.check_user(&body.username, &body.password).await?;
    if is_valid {
        user::add_user_cookie(cookies, body.username.to_string());
        Ok(Json(()))
    } else {
        Err(HttpError::new(401, "User not authorized"))
    }
}


#[post("/api/logout")]
async fn logout(cookies: &CookieJar<'_>) -> HttpResult<Json<()>> {
    user::remove_user_cookie(cookies);
    Ok(Json(()))
}


#[derive(Deserialize)]
struct UpsertUserInput {
    username: String,
    password: String
}

#[post("/api/upsert_user", data = "<body>")]
async fn upsert_user(_user: User, store: State<'_, PersistedStore>, body: Json<UpsertUserInput>) -> HttpResult<Json<()>> {
    let new_user = body.into_inner();
    let hashed_password = HashedPassword::from_plain_password(&new_user.password);
    store.upsert_user(new_user.username, hashed_password).await?;
    Ok(Json(()))
}
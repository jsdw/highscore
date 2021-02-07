use crate::store_interface::{ HashedPassword, Store, GroupId, ScorableId, ScoreId, Group, Scorable, Score };
use crate::persisted_store::{ PersistedStore };
use crate::user::{ self, User };
use crate::http_result::{ HttpError, HttpResult };
use serde::{ Serialize, Deserialize };
use rocket_contrib::json::Json;
use rocket::{State, http::CookieJar};
use chrono::{ DateTime, Utc };

/// Ths entrypoint; gather the routes decalred below:
pub fn routes() -> Vec<rocket::Route> {
    routes![
        login,
        logout,
        upsert_user,
        delete_user,
        upsert_group,
        delete_group,
        upsert_scorable,
        delete_scorable,
        upsert_score,
        delete_score,
        groups,
        scorables_in_group,
        scores,
    ]
}

#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String
}

#[derive(Serialize)]
struct Empty {}

#[post("/login", data = "<body>")]
async fn login(store: State<'_, PersistedStore>, cookies: &CookieJar<'_>, body: Json<LoginInput>) -> HttpResult<Json<Empty>> {
    let is_valid = store.check_user(&body.username, &body.password).await?;
    if is_valid {
        user::add_user_cookie(cookies, body.username.to_string());
        Ok(Json(Empty {}))
    } else {
        Err(HttpError::new(401, "User not authorized"))
    }
}


#[post("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> HttpResult<Json<Empty>> {
    user::remove_user_cookie(cookies);
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct UpsertUserInput {
    username: String,
    password: String
}

#[post("/upsert_user", data = "<body>")]
async fn upsert_user(_user: User, store: State<'_, PersistedStore>, body: Json<UpsertUserInput>) -> HttpResult<Json<Empty>> {
    let new_user = body.into_inner();
    let hashed_password = HashedPassword::from_plain_password(&new_user.password);
    store.upsert_user(new_user.username, hashed_password).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct DeleteUserInput {
    username: String,
}

#[post("/delete_user", data = "<body>")]
async fn delete_user(_user: User, store: State<'_, PersistedStore>, body: Json<DeleteUserInput>) -> HttpResult<Json<Empty>> {
    store.delete_user(&body.username).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct UpsertGroupInput {
    id: GroupId,
    name: String
}

#[post("/upsert_group", data = "<body>")]
async fn upsert_group(_user: User, store: State<'_, PersistedStore>, body: Json<UpsertGroupInput>) -> HttpResult<Json<Empty>> {
    let group = body.into_inner();
    store.upsert_group(group.id, group.name).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct DeleteGroupInput {
    id: GroupId,
}

#[post("/delete_group", data = "<body>")]
async fn delete_group(_user: User, store: State<'_, PersistedStore>, body: Json<DeleteGroupInput>) -> HttpResult<Json<Empty>> {
    store.delete_group(&body.id).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct UpsertScorableInput {
    id: ScorableId,
    group_id: GroupId,
    name: String
}

#[post("/upsert_scorable", data = "<body>")]
async fn upsert_scorable(_user: User, store: State<'_, PersistedStore>, body: Json<UpsertScorableInput>) -> HttpResult<Json<Empty>> {
    let scorable = body.into_inner();
    store.upsert_scorable(scorable.id, scorable.group_id, scorable.name).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct DeleteScorableInput {
    id: ScorableId,
}

#[post("/delete_scorable", data = "<body>")]
async fn delete_scorable(_user: User, store: State<'_, PersistedStore>, body: Json<DeleteScorableInput>) -> HttpResult<Json<Empty>> {
    store.delete_scorable(&body.id).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct UpsertScoreInput {
    id: ScoreId,
    scorable_id: ScorableId,
    username: String,
    value: i64,
    date: DateTime<Utc>
}

#[post("/upsert_score", data = "<body>")]
async fn upsert_score(_user: User, store: State<'_, PersistedStore>, body: Json<UpsertScoreInput>) -> HttpResult<Json<Empty>> {
    let score = body.into_inner();
    store.upsert_score(score.id, score.scorable_id, score.username, score.value, score.date).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct DeleteScoreInput {
    id: ScoreId,
}

#[post("/delete_score", data = "<body>")]
async fn delete_score(_user: User, store: State<'_, PersistedStore>, body: Json<DeleteScoreInput>) -> HttpResult<Json<Empty>> {
    store.delete_score(&body.id).await?;
    Ok(Json(Empty {}))
}


#[get("/groups")]
async fn groups(_user: User, store: State<'_, PersistedStore>) -> HttpResult<Json<Vec<Group>>> {
    let groups = store.groups().await?;
    Ok(Json(groups))
}


#[derive(Deserialize)]
struct ScorablesInGroupInput {
    group_id: GroupId,
}

#[post("/scorables_in_group", data = "<body>")]
async fn scorables_in_group(_user: User, store: State<'_, PersistedStore>, body: Json<ScorablesInGroupInput>) -> HttpResult<Json<Vec<Scorable>>> {
    let scorables = store.scorables_in_group(&body.group_id).await?;
    Ok(Json(scorables))
}


#[derive(Deserialize)]
struct ScoresInput {
    scorable_id: ScorableId,
    limit: Option<usize>
}

#[get("/scores", data = "<body>")]
async fn scores(_user: User, store: State<'_, PersistedStore>, body: Json<ScoresInput>) -> HttpResult<Json<Vec<Score>>> {
    let scores = store.scores(&body.scorable_id, body.limit.clone()).await?;
    Ok(Json(scores))
}
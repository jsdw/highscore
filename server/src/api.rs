//! Rocket API routes to provide access to the backend.

use crate::store_interface::{ HashedPassword, GroupId, ScorableId, ScoreId, Group, Scorable, Score };
use crate::user::{ self, User };
use crate::http_result::{ HttpError, HttpResult };
use crate::state;
use serde::{ Serialize, Deserialize };
use rocket_contrib::json::Json;
use rocket::{State, http::CookieJar};
use chrono::{ DateTime, Utc };

/// Ths entrypoint; gather the routes decalred below:
pub fn routes() -> Vec<rocket::Route> {
    routes![
        last_changed,
        login,
        logout,
        current_user,
        upsert_user,
        delete_user,
        upsert_group,
        delete_group,
        get_group,
        upsert_scorable,
        delete_scorable,
        get_scorable,
        upsert_score,
        delete_score,
        groups,
        scorables_in_group,
        scores,
    ]
}


#[derive(Serialize)]
struct LastChangedOutput {
    date: DateTime<Utc>
}

#[get("/last_changed")]
async fn last_changed(_user: User, state: State<'_, state::State>) -> HttpResult<Json<LastChangedOutput>> {
    let date = state.store.last_changed().await;
    Ok(Json(LastChangedOutput { date }))
}


#[derive(Deserialize)]
struct LoginInput {
    username: String,
    password: String
}

#[derive(Serialize)]
struct Empty {}

#[post("/login", data = "<body>")]
async fn login(state: State<'_, state::State>, cookies: &CookieJar<'_>, body: Json<LoginInput>) -> HttpResult<Json<Empty>> {
    let is_valid = state.store.check_user(&body.username, &body.password).await?;
    if is_valid {
        user::add_user_cookie(cookies, body.username.to_string());
        Ok(Json(Empty {}))
    } else {
        Err(HttpError::new(401, "User not authorized"))
    }
}


#[get("/logout")]
async fn logout(cookies: &CookieJar<'_>) -> HttpResult<Json<Empty>> {
    user::remove_user_cookie(cookies);
    Ok(Json(Empty {}))
}


#[derive(Serialize)]
struct CurrentUserOutput {
    username: Option<String>
}

#[get("/current_user")]
async fn current_user(user: Option<User>) -> HttpResult<Json<CurrentUserOutput>> {
    Ok(Json(CurrentUserOutput { username: user.map(|u| u.name) }))
}


#[derive(Deserialize)]
struct UpsertUserInput {
    username: Option<String>,
    password: String
}

#[post("/upsert_user", data = "<body>")]
async fn upsert_user(user: User, state: State<'_, state::State>, body: Json<UpsertUserInput>) -> HttpResult<Json<Empty>> {
    let new_user = body.into_inner();
    let username = new_user.username.unwrap_or_else(|| user.name.to_owned());
    let plain_password = new_user.password;
    let hashed_password = tokio::task::spawn_blocking(move || HashedPassword::from_plain_password(&plain_password))
        .await
        .map_err(|_| HttpError::server_error("Failed to join thread after hashing password"))?;
    state.store.upsert_user(username, hashed_password).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct DeleteUserInput {
    username: String,
}

#[post("/delete_user", data = "<body>")]
async fn delete_user(_user: User, state: State<'_, state::State>, body: Json<DeleteUserInput>) -> HttpResult<Json<Empty>> {
    state.store.delete_user(&body.username).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct UpsertGroupInput {
    id: Option<GroupId>,
    name: String
}

#[derive(Serialize)]
struct GroupOutput {
    id: GroupId,
    name: String
}

#[post("/upsert_group", data = "<body>")]
async fn upsert_group(_user: User, state: State<'_, state::State>, body: Json<UpsertGroupInput>) -> HttpResult<Json<GroupOutput>> {
    let group = body.into_inner();
    let id = group.id.unwrap_or_else(GroupId::new);
    state.store.upsert_group(id, group.name.clone()).await?;
    Ok(Json(GroupOutput { id, name: group.name }))
}


#[derive(Deserialize)]
struct GetGroupInput {
    id: GroupId
}

#[post("/get_group", data = "<body>")]
async fn get_group(_user: User, state: State<'_, state::State>, body: Json<GetGroupInput>) -> HttpResult<Json<GroupOutput>> {
    let group = state.store.get_group(&body.id).await?;
    Ok(Json(GroupOutput { id: group.id, name: group.name }))
}


#[derive(Deserialize)]
struct DeleteGroupInput {
    id: GroupId,
}

#[post("/delete_group", data = "<body>")]
async fn delete_group(_user: User, state: State<'_, state::State>, body: Json<DeleteGroupInput>) -> HttpResult<Json<Empty>> {
    state.store.delete_group(&body.id).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct UpsertScorableInput {
    id: Option<ScorableId>,
    group_id: GroupId,
    name: String
}

#[derive(Serialize)]
struct ScorableOutput {
    id: ScorableId,
    name: String
}

#[post("/upsert_scorable", data = "<body>")]
async fn upsert_scorable(_user: User, state: State<'_, state::State>, body: Json<UpsertScorableInput>) -> HttpResult<Json<ScorableOutput>> {
    let scorable = body.into_inner();
    let id = scorable.id.unwrap_or_else(ScorableId::new);
    state.store.upsert_scorable(id, scorable.group_id, scorable.name.clone()).await?;
    Ok(Json(ScorableOutput { id, name: scorable.name }))
}


#[derive(Deserialize)]
struct DeleteScorableInput {
    id: ScorableId,
}

#[post("/delete_scorable", data = "<body>")]
async fn delete_scorable(_user: User, state: State<'_, state::State>, body: Json<DeleteScorableInput>) -> HttpResult<Json<Empty>> {
    state.store.delete_scorable(&body.id).await?;
    Ok(Json(Empty {}))
}


#[derive(Deserialize)]
struct GetScorableInput {
    id: ScorableId
}

#[post("/get_scorable", data = "<body>")]
async fn get_scorable(_user: User, state: State<'_, state::State>, body: Json<GetScorableInput>) -> HttpResult<Json<ScorableOutput>> {
    let scorable = state.store.get_scorable(&body.id).await?;
    Ok(Json(ScorableOutput { id: scorable.id, name: scorable.name }))
}


#[derive(Deserialize)]
struct UpsertScoreInput {
    id: Option<ScoreId>,
    scorable_id: ScorableId,
    username: Option<String>,
    value: i64,
    date: Option<DateTime<Utc>>
}

#[derive(Serialize)]
struct UpsertScoreOutput {
    id: ScoreId
}

#[post("/upsert_score", data = "<body>")]
async fn upsert_score(user: User, state: State<'_, state::State>, body: Json<UpsertScoreInput>) -> HttpResult<Json<UpsertScoreOutput>> {
    let score = body.into_inner();
    let date = score.date.unwrap_or_else(|| Utc::now());
    let username = score.username.unwrap_or_else(|| user.name.to_owned());
    let id = score.id.unwrap_or_else(ScoreId::new);
    state.store.upsert_score(id, score.scorable_id, username, score.value, date).await?;
    Ok(Json(UpsertScoreOutput { id }))
}


#[derive(Deserialize)]
struct DeleteScoreInput {
    id: ScoreId,
}

#[post("/delete_score", data = "<body>")]
async fn delete_score(_user: User, state: State<'_, state::State>, body: Json<DeleteScoreInput>) -> HttpResult<Json<Empty>> {
    state.store.delete_score(&body.id).await?;
    Ok(Json(Empty {}))
}


#[get("/groups")]
async fn groups(_user: User, state: State<'_, state::State>) -> HttpResult<Json<Vec<Group>>> {
    let groups = state.store.groups().await?;
    Ok(Json(groups))
}


#[derive(Deserialize)]
struct ScorablesInGroupInput {
    group_id: GroupId,
}

#[post("/scorables_in_group", data = "<body>")]
async fn scorables_in_group(_user: User, state: State<'_, state::State>, body: Json<ScorablesInGroupInput>) -> HttpResult<Json<Vec<Scorable>>> {
    let scorables = state.store.scorables_in_group(&body.group_id).await?;
    Ok(Json(scorables))
}


#[derive(Deserialize)]
struct ScoresInput {
    scorable_id: ScorableId,
    limit: Option<usize>
}

#[post("/scores", data = "<body>")]
async fn scores(_user: User, state: State<'_, state::State>, body: Json<ScoresInput>) -> HttpResult<Json<Vec<Score>>> {
    let scores = state.store.scores(&body.scorable_id, body.limit.clone()).await?;
    Ok(Json(scores))
}
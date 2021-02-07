//! Describe the shape of the scores interface that we expect to interact with.
//! This will roughly correspond to the JSON API.
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use std::{fmt, hash::Hash};
use chrono::prelude::{ DateTime, Utc };
use std::fmt::Display;

#[async_trait::async_trait]
pub trait Store {
    /// We can render errors, and know whether to show themn to users or not
    type Error: Display + HasErrorKind;

    /// Add/update a user
    async fn upsert_user(&self, username: String, password: HashedPassword) -> Result<(),Self::Error>;
    /// Check that a user exists with the password provided
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,Self::Error>;
    /// Delete a user
    async fn delete_user(&self, username: &str) -> Result<(),Self::Error>;

    /// Add/update a group
    async fn upsert_group(&self, id: GroupId, name: String) -> Result<(),Self::Error>;
    /// Delete a group
    async fn delete_group(&self, id: &GroupId) -> Result<(),Self::Error>;

    /// Add/update a thing to save scores against
    async fn upsert_scorable(&self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),Self::Error>;
    /// Delete a scorable
    async fn delete_scorable(&self, id: &ScorableId) -> Result<(),Self::Error>;

    /// Add/update a score against something
    async fn upsert_score(&self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: DateTime<Utc>) -> Result<(),Self::Error>;
    /// Delete a score against something
    async fn delete_score(&self, id: &ScoreId) -> Result<(),Self::Error>;

    /// Return a list of groups that we know about
    async fn groups(&self) -> Result<Vec<Group>,Self::Error>;
    /// Return a list of scorable things in a group
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<Scorable>,Self::Error>;
    /// Return a list of scores for a scorable thing (highest first, up to some limit)
    async fn scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<Score>,Self::Error>;

}

pub trait HasErrorKind {
    fn error_kind(&self) -> ErrorKind;
}

#[derive(Clone,Copy,PartialEq,Eq)]
pub enum ErrorKind {
    /// The error was caused by the user (eg trying to set a score
    /// for a scorable that doesn't exist)
    UserError,
}

#[derive(Debug,Serialize,Clone)]
pub struct Group {
    pub id: GroupId,
    pub name: String
}

#[derive(Debug,Serialize,Clone)]
pub struct Scorable {
    pub id: ScorableId,
    pub name: String
}

#[derive(Debug,Serialize,Clone)]
pub struct Score {
    pub id: ScoreId,
    pub username: String,
    pub value: i64,
    pub date: DateTime<Utc>
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct GroupId(Uuid);

impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct ScorableId(Uuid);

impl fmt::Display for ScorableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct ScoreId(Uuid);

impl fmt::Display for ScoreId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
pub struct HashedPassword(String);

impl HashedPassword {
    pub fn from_plain_password(plain: &str) -> HashedPassword {
        HashedPassword(crate::password::hash(plain))
    }
    pub fn verify_plain_password(&self, plain: &str) -> bool {
        crate::password::verify(&self.0, plain)
    }
}
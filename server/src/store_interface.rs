//! Describe the shape of the scores interface that we expect to interact with.
//! This will roughly correspond to the JSON API.
use serde::{ Serialize, Deserialize };
use uuid::Uuid;
use std::{fmt, hash::Hash};
use chrono::prelude::{ DateTime, Utc };

#[async_trait::async_trait]
pub trait Store {
    /// Add/update a user
    async fn upsert_user(&self, username: String, password: HashedPassword) -> Result<(),StoreError>;
    /// Check that a user exists with the password provided
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,StoreError>;
    /// Delete a user
    async fn delete_user(&self, username: &str) -> Result<(),StoreError>;

    /// Add/update a group
    async fn upsert_group(&self, id: GroupId, name: String) -> Result<Group,StoreError>;
    /// Delete a group
    async fn delete_group(&self, id: &GroupId) -> Result<(),StoreError>;
    /// Get a group
    async fn get_group(&self, id: &GroupId) -> Result<Group,StoreError>;

    /// Add/update a thing to save scores against
    async fn upsert_scorable(&self, id: ScorableId, group_id: GroupId, name: String) -> Result<Scorable,StoreError>;
    /// Delete a scorable
    async fn delete_scorable(&self, id: &ScorableId) -> Result<(),StoreError>;
    /// Get a scorable
    async fn get_scorable(&self, id: &ScorableId) -> Result<Scorable,StoreError>;

    /// Add/update a score against something
    async fn upsert_score(&self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: Option<DateTime<Utc>>) -> Result<Score,StoreError>;
    /// Delete a score against something
    async fn delete_score(&self, id: &ScoreId) -> Result<(),StoreError>;

    /// Return a list of groups that we know about
    async fn groups(&self) -> Result<Vec<Group>,StoreError>;
    /// Return a list of scorable things in a group
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<Scorable>,StoreError>;
    /// Return a list of scores for a scorable thing (highest first, up to some limit)
    async fn scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<Score>,StoreError>;

}

#[derive(thiserror::Error,Debug)]
pub enum StoreError {
    #[error("user '{0}' not found")]
    UserNotFound(String),
    #[error("group '{0}' not found")]
    GroupNotFound(GroupId),
    #[error("scorable '{0}' not found")]
    ScorableNotFound(ScorableId),
    #[error("score '{0}' not found")]
    ScoreNotFound(ScoreId)
}

#[derive(Debug,Serialize,Clone,Ord,PartialOrd,Eq,PartialEq)]
pub struct Group {
    pub id: GroupId,
    pub name: String
}

#[derive(Debug,Serialize,Clone,Ord,PartialOrd,Eq,PartialEq)]
pub struct Scorable {
    // Name comes first for Ord impl:
    pub name: String,
    pub id: ScorableId,
}

#[derive(Debug,Serialize,Clone)]
pub struct Score {
    pub id: ScoreId,
    pub username: String,
    pub value: i64,
    pub date: DateTime<Utc>
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,PartialOrd,Ord,Debug,Clone,Copy)]
pub struct GroupId(Uuid);

impl GroupId {
    pub fn new() -> GroupId {
        GroupId(Uuid::new_v4())
    }
}
impl fmt::Display for GroupId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,PartialOrd,Ord,Debug,Clone,Copy)]
pub struct ScorableId(Uuid);

impl ScorableId {
    pub fn new() -> ScorableId {
        ScorableId(Uuid::new_v4())
    }
}
impl fmt::Display for ScorableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,PartialOrd,Ord,Debug,Clone,Copy)]
pub struct ScoreId(Uuid);

impl ScoreId {
    pub fn new() -> ScoreId {
        ScoreId(Uuid::new_v4())
    }
}
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
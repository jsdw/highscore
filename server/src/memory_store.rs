//! Our in-memory store can be shared across threads, and provides a simple
//! implementation of [`crate::store_interface::Store`]. We prefer to use the
//! persisted_store though, which builds persistence on top of this.

use std::{collections::HashMap, sync::MutexGuard};
use std::sync::Mutex;
use chrono::prelude::{ DateTime, Utc };
use crate::events::{ Event, EventHandler };
use crate::store_interface::{ self, Store, GroupId, ScorableId, ScoreId, HashedPassword };
use thiserror::Error;

#[derive(Error,Debug)]
pub enum MemoryStoreError {
    #[error("user '{0}' not found")]
    UserNotFound(String),
    #[error("group '{0}' not found")]
    GroupNotFound(GroupId),
    #[error("scorable '{0}' not found")]
    ScorableNotFound(ScorableId),
    #[error("score '{0}' not found")]
    ScoreNotFound(ScoreId)
}

impl store_interface::HasErrorKind for MemoryStoreError {
    fn error_kind(&self) -> store_interface::ErrorKind {
        store_interface::ErrorKind::UserError
    }
}
pub struct MemoryStore {
    inner: Mutex<MemoryStoreInner>
}

struct MemoryStoreInner {
    /// Users (mapping of username to password)
    users: HashMap<String, HashedPassword>,
    /// Groups of scorables that themselves have scores on
    scores: HashMap<GroupId, Group>,
    // Indexes:
    scorable_to_group: HashMap<ScorableId, GroupId>,
    score_to_scorable: HashMap<ScoreId, ScorableId>
}

impl MemoryStore {
    /// Load data in from persisted events.
    pub async fn from_events(events: &EventHandler) -> anyhow::Result<MemoryStore> {
        use futures::stream::StreamExt;
        let mut data = MemoryStoreInner {
            users: HashMap::new(),
            scores: HashMap::new(),
            // Indexes:
            scorable_to_group: HashMap::new(),
            score_to_scorable: HashMap::new(),
        };
        let mut event_stream = events.read_from_disk().await?;
        while let Some(event) = event_stream.next().await {
            let event = event?;
            match event {
                Event::UpsertUser { username, hashed_password } => {
                    data.users.insert(username, hashed_password);
                }
                Event::DeleteUser { username } => {
                    if let Err(e) = data.delete_user(&username) {
                        log::warn!("Ignoring event DeleteUser: {}", e);
                    }
                }
                Event::UpsertGroup { id, name } => {
                    if let Err(e) = data.upsert_group(id, name) {
                        log::warn!("Ignoring event UpsertGroup: {}", e);
                    }
                }
                Event::DeleteGroup { id } => {
                    if let Err(e) = data.delete_group(&id) {
                        log::warn!("Ignoring event DeleteGroup: {}", e);
                    }
                }
                Event::UpsertScorable { id, group_id, name } => {
                    if let Err(e) = data.upsert_scorable(id, group_id, name) {
                        log::warn!("Ignoring event AddScorable: {}", e);
                    }
                }
                Event::DeleteScorable { id } => {
                    if let Err(e) = data.delete_scorable(&id) {
                        log::warn!("Ignoring event DeleteScorable: {}", e);
                    }
                }
                Event::UpsertScore { id, scorable_id, username, value, date } => {
                    if let Err(e) = data.upsert_score(id, scorable_id, username, value, date) {
                        log::warn!("Ignoring event AddScore: {}", e);
                    }
                }
                Event::DeleteScore { id } => {
                    if let Err(e) = data.delete_score(&id) {
                        log::warn!("Ignoring event DeleteScore: {}", e);
                    }
                }
            }
        }
        Ok(MemoryStore { inner: Mutex::new(data) })
    }
    // A convenience to lock the inner store briefly so that we can call things against it.
    fn lock(&self) -> MutexGuard<MemoryStoreInner> {
        self.inner.lock().unwrap()
    }
}

// MemoryStore is a valid store on its own, but it's mainly used as
// the in-memory part of persisted_store.
#[async_trait::async_trait]
impl Store for MemoryStore {
    type Error = MemoryStoreError;

    async fn upsert_user(&self, username: String, password: HashedPassword) -> Result<(),Self::Error> {
        self.lock().upsert_user(username, password)
    }
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,Self::Error> {
        self.lock().check_user(username, password)
    }
    async fn delete_user(&self, username: &str) -> Result<(),Self::Error> {
        self.lock().delete_user(username)
    }

    async fn upsert_group(&self, id: GroupId, name: String) -> Result<(),Self::Error> {
        self.lock().upsert_group(id, name)
    }
    async fn delete_group(&self, id: &GroupId) -> Result<(),Self::Error> {
        self.lock().delete_group(id)
    }

    async fn upsert_scorable(&self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),Self::Error> {
        self.lock().upsert_scorable(id, group_id, name)
    }
    async fn delete_scorable(&self, id: &ScorableId) -> Result<(),Self::Error> {
        self.lock().delete_scorable(id)
    }

    async fn upsert_score(&self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: DateTime<Utc>) -> Result<(),Self::Error> {
        self.lock().upsert_score(id, scorable_id, username, value, date)
    }
    async fn delete_score(&self, id: &ScoreId) -> Result<(),Self::Error> {
        self.lock().delete_score(id)
    }

    async fn groups(&self) -> Result<Vec<crate::store_interface::Group>,Self::Error> {
        self.lock().groups()
    }
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<store_interface::Scorable>,Self::Error> {
        self.lock().scorables_in_group(group_id)
    }
    async fn scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<store_interface::Score>,Self::Error> {
        self.lock().get_scores(scorable_id, limit)
    }
}

impl MemoryStoreInner {
    // Working with Users
    pub fn upsert_user(&mut self, username: String, hashed_password: HashedPassword) -> Result<(),MemoryStoreError> {
        self.users.insert(username, hashed_password);
        Ok(())
    }
    pub fn check_user(&mut self, username: &str, password: &str) -> Result<bool,MemoryStoreError> {
        let is_valid = self.users
            .get(username)
            .map(|hash| hash.verify_plain_password(password))
            .unwrap_or(false);
        Ok(is_valid)
    }
    pub fn delete_user(&mut self, username: &str) -> Result<(),MemoryStoreError> {
        self.users.remove(username)
            .ok_or_else(|| MemoryStoreError::UserNotFound(username.to_owned()))?;
        // Remove all scores associated with this user, too:
        for group in self.scores.values_mut() {
            for scores in group.scorables.values_mut() {
                scores.scores.retain(|_,s| s.username != username);
            }
        }
        Ok(())
    }

    // Editing Groups
    pub fn upsert_group(&mut self, id: GroupId, name: String) -> Result<(),MemoryStoreError> {
        self.scores
            .entry(id)
            .or_insert_with(|| Group::empty())
            .name = name;
        Ok(())
    }
    pub fn delete_group(&mut self, id: &GroupId) -> Result<(),MemoryStoreError> {
        self.scores.remove(id)
            .ok_or(MemoryStoreError::GroupNotFound(*id))
            .map(|_| ())
    }

    // Editing Scorables
    pub fn upsert_scorable(&mut self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),MemoryStoreError> {
        if let Some(group) = self.scores.get_mut(&group_id) {
            group.scorables
                .entry(id)
                .or_insert_with(|| Scorable::empty())
                .name = name;
            self.scorable_to_group.insert(id, group_id);
            Ok(())
        } else {
            Err(MemoryStoreError::GroupNotFound(group_id))
        }
    }
    pub fn delete_scorable(&mut self, id: &ScorableId) -> Result<(),MemoryStoreError> {
        let group_id = self.scorable_to_group
            .remove(id)
            .ok_or(MemoryStoreError::ScorableNotFound(*id))?;
        self.scores.get_mut(&group_id)
            .ok_or(MemoryStoreError::GroupNotFound(group_id))?
            .scorables.remove(&id)
            .ok_or(MemoryStoreError::ScorableNotFound(*id))
            .map(|_| ())
    }

    // Editing Scores
    pub fn upsert_score(&mut self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: DateTime<Utc>) -> Result<(),MemoryStoreError> {
        if !self.users.contains_key(&username) {
            return Err(MemoryStoreError::UserNotFound(username));
        }
        let group_id = self.scorable_to_group.get(&scorable_id)
            .ok_or(MemoryStoreError::ScorableNotFound(scorable_id))?;
        let group = self.scores.get_mut(group_id)
            .ok_or(MemoryStoreError::GroupNotFound(*group_id))?;
        if let Some(scorable) = group.scorables.get_mut(&scorable_id) {
            scorable.scores.insert(id, Score { username, value, date });
            self.score_to_scorable.insert(id, scorable_id);
            Ok(())
        } else {
            Err(MemoryStoreError::ScorableNotFound(scorable_id))
        }
    }
    pub fn delete_score(&mut self, id: &ScoreId) -> Result<(),MemoryStoreError> {
        let scorable_id = self.score_to_scorable.remove(id)
            .ok_or(MemoryStoreError::ScoreNotFound(*id))?;
        let group_id = self.scorable_to_group.get(&scorable_id)
            .ok_or(MemoryStoreError::ScorableNotFound(scorable_id))?;
        let group = self.scores.get_mut(group_id)
            .ok_or(MemoryStoreError::GroupNotFound(*group_id))?;
        group.scorables.get_mut(&scorable_id)
            .ok_or(MemoryStoreError::ScorableNotFound(scorable_id))?
            .scores.remove(id)
            .ok_or(MemoryStoreError::ScoreNotFound(*id))
            .map(|_| ())
    }

    pub fn groups(&self) -> Result<Vec<crate::store_interface::Group>,MemoryStoreError> {
        let groups = self.scores
            .iter()
            .map(|(id,group)| store_interface::Group {
                id: *id,
                name: group.name.to_owned()
            })
            .collect();
        Ok(groups)
    }
    pub fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<crate::store_interface::Scorable>,MemoryStoreError> {
        let scorables = self.scores.get(&group_id)
            .ok_or(MemoryStoreError::GroupNotFound(*group_id))?
            .iter_scorables()
            .map(|(id,scorable)| store_interface::Scorable {
                id: id,
                name: scorable.name.to_owned()
            })
            .collect();
        Ok(scorables)
    }
    pub fn get_scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<crate::store_interface::Score>,MemoryStoreError> {
        let group_id = self.scorable_to_group.get(&scorable_id)
            .ok_or(MemoryStoreError::ScorableNotFound(*scorable_id))?;
        let group = self.scores.get(group_id)
            .ok_or(MemoryStoreError::GroupNotFound(*group_id))?;
        let mut scores: Vec<_> = group.scorables.get(scorable_id)
            .ok_or(MemoryStoreError::ScorableNotFound(*scorable_id))?
            .scores.iter()
            .collect();
        // highest score first:
        scores.sort_by_key(|(_,s)| std::cmp::Reverse(s.value));
        let limit = limit.unwrap_or(scores.len());
        let scores = scores.into_iter()
            .take(limit)
            .map(|(id,s)| store_interface::Score {
                id: *id,
                date: s.date,
                username: s.username.clone(),
                value: s.value
            })
            .collect();
        Ok(scores)
    }
}

struct Group {
    name: String,
    scorables: HashMap<ScorableId, Scorable>
}

impl Group {
    fn empty() -> Group {
        Group { name: String::new(), scorables: HashMap::new() }
    }
    fn iter_scorables(&self) -> impl Iterator<Item=(ScorableId,&Scorable)> + '_ {
        self.scorables.iter().map(|(id,scorable)| (*id,scorable))
    }
}

struct Scorable {
    name: String,
    scores: HashMap<ScoreId, Score>
}

impl Scorable {
    fn empty() -> Scorable {
        Scorable { name: String::new(), scores: HashMap::new() }
    }
}

struct Score {
    username: String,
    value: i64,
    date: DateTime<Utc>
}

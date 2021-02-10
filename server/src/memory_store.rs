//! Our in-memory store can be shared across threads, and provides a simple
//! implementation of [`crate::store_interface::Store`]. We prefer to use the
//! persisted_store though, which builds persistence on top of this.

use std::{collections::HashMap, sync::MutexGuard};
use std::sync::Mutex;
use chrono::prelude::{ DateTime, Utc };
use crate::events::{ Event, EventHandler };
use crate::store_interface::{ self, Store, StoreError, GroupId, ScorableId, ScoreId, HashedPassword };

pub struct MemoryStore {
    inner: Mutex<MemoryStoreInner>
}

struct MemoryStoreInner {
    /// When was the last change made? This may update despite
    /// no changes being made, but must update if changes are made.
    last_changed: DateTime<Utc>,
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
            last_changed: Utc::now(),
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
    async fn last_changed(&self) -> DateTime<Utc> {
        self.lock().last_changed()
    }

    async fn users(&self) -> Result<Vec<String>,StoreError> {
        self.lock().users()
    }
    async fn upsert_user(&self, username: String, password: HashedPassword) -> Result<(),StoreError> {
        self.lock().upsert_user(username, password)
    }
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,StoreError> {
        let hashed_password = self.lock()
            .get_hashed_password(username)
            .ok_or_else(|| StoreError::UserNotFound(username.to_owned()))?;
        let res = tokio::task::block_in_place(||
            hashed_password.verify_plain_password(password)
        );
        Ok(res)
    }
    async fn delete_user(&self, username: &str) -> Result<(),StoreError> {
        self.lock().delete_user(username)
    }

    async fn upsert_group(&self, id: GroupId, name: String) -> Result<(),StoreError> {
        self.lock().upsert_group(id, name)
    }
    async fn delete_group(&self, id: &GroupId) -> Result<(),StoreError> {
        self.lock().delete_group(id)
    }
    async fn get_group(&self, id: &GroupId) -> Result<store_interface::Group,StoreError> {
        self.lock().get_group(id)
    }

    async fn upsert_scorable(&self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),StoreError> {
        self.lock().upsert_scorable(id, group_id, name)
    }
    async fn delete_scorable(&self, id: &ScorableId) -> Result<(),StoreError> {
        self.lock().delete_scorable(id)
    }
    async fn get_scorable(&self, id: &ScorableId) -> Result<store_interface::Scorable,StoreError> {
        self.lock().get_scorable(id)
    }

    async fn upsert_score(&self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: DateTime<Utc>) -> Result<(),StoreError> {
        self.lock().upsert_score(id, scorable_id, username, value, date)
    }
    async fn delete_score(&self, id: &ScoreId) -> Result<(),StoreError> {
        self.lock().delete_score(id)
    }

    async fn groups(&self) -> Result<Vec<crate::store_interface::Group>,StoreError> {
        self.lock().groups()
    }
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<store_interface::Scorable>,StoreError> {
        self.lock().scorables_in_group(group_id)
    }
    async fn scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<store_interface::Score>,StoreError> {
        self.lock().get_scores(scorable_id, limit)
    }
}

impl MemoryStoreInner {
    // Working with Users
    pub fn users(&self) -> Result<Vec<String>,StoreError> {
        Ok(self.users.keys().map(|u| u.to_owned()).collect())
    }
    pub fn upsert_user(&mut self, username: String, hashed_password: HashedPassword) -> Result<(),StoreError> {
        self.update_last_changed();
        self.users.insert(username, hashed_password);
        Ok(())
    }
    pub fn get_hashed_password(&self, username: &str) -> Option<HashedPassword> {
        self.users.get(username).map(|u| u.clone())
    }
    pub fn delete_user(&mut self, username: &str) -> Result<(),StoreError> {
        self.update_last_changed();
        self.users.remove(username)
            .ok_or_else(|| StoreError::UserNotFound(username.to_owned()))?;
        // Remove all scores associated with this user, too:
        for group in self.scores.values_mut() {
            for scores in group.scorables.values_mut() {
                scores.scores.retain(|_,s| s.username != username);
            }
        }
        Ok(())
    }

    // Editing Groups
    pub fn upsert_group(&mut self, id: GroupId, name: String) -> Result<(),StoreError> {
        self.update_last_changed();
        self.scores
            .entry(id)
            .or_insert_with(|| Group::empty())
            .name = name;
        Ok(())
    }
    pub fn delete_group(&mut self, id: &GroupId) -> Result<(),StoreError> {
        self.update_last_changed();
        self.scores.remove(id)
            .ok_or(StoreError::GroupNotFound(*id))
            .map(|_| ())
    }
    pub fn get_group(&self, id: &GroupId) -> Result<store_interface::Group,StoreError> {
        self.scores.get(id)
            .map(|g| store_interface::Group { id: *id, name: g.name.to_owned() })
            .ok_or(StoreError::GroupNotFound(*id))
    }

    // Editing Scorables
    pub fn upsert_scorable(&mut self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),StoreError> {
        self.update_last_changed();
        if let Some(group) = self.scores.get_mut(&group_id) {
            group.scorables
                .entry(id)
                .or_insert_with(|| Scorable::empty())
                .name = name;
            self.scorable_to_group.insert(id, group_id);
            Ok(())
        } else {
            Err(StoreError::GroupNotFound(group_id))
        }
    }
    pub fn delete_scorable(&mut self, id: &ScorableId) -> Result<(),StoreError> {
        let group_id = self.scorable_to_group
        .remove(id)
        .ok_or(StoreError::ScorableNotFound(*id))?;
        self.update_last_changed();
        self.scores.get_mut(&group_id)
            .ok_or(StoreError::GroupNotFound(group_id))?
            .scorables.remove(&id)
            .ok_or(StoreError::ScorableNotFound(*id))
            .map(|_| ())
    }
    pub fn get_scorable(&self, id: &ScorableId) -> Result<store_interface::Scorable,StoreError> {
        let group_id = self.scorable_to_group.get(id)
            .ok_or(StoreError::ScorableNotFound(*id))?;
        let group = self.scores.get(group_id)
            .ok_or(StoreError::GroupNotFound(*group_id))?;
        group.scorables.get(id)
            .map(|s| store_interface::Scorable { id: *id, name: s.name.to_owned() })
            .ok_or(StoreError::ScorableNotFound(*id))
    }

    // Editing Scores
    pub fn upsert_score(&mut self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: DateTime<Utc>) -> Result<(),StoreError> {
        if !self.users.contains_key(&username) {
            return Err(StoreError::UserNotFound(username));
        }
        self.update_last_changed();
        let group_id = self.scorable_to_group.get(&scorable_id)
            .ok_or(StoreError::ScorableNotFound(scorable_id))?;
        let group = self.scores.get_mut(group_id)
            .ok_or(StoreError::GroupNotFound(*group_id))?;
        if let Some(scorable) = group.scorables.get_mut(&scorable_id) {
            scorable.scores.insert(id, Score { username: username.clone(), value, date });
            self.score_to_scorable.insert(id, scorable_id);
            Ok(())
        } else {
            Err(StoreError::ScorableNotFound(scorable_id))
        }
    }
    pub fn delete_score(&mut self, id: &ScoreId) -> Result<(),StoreError> {
        self.update_last_changed();
        let scorable_id = self.score_to_scorable.remove(id)
            .ok_or(StoreError::ScoreNotFound(*id))?;
        let group_id = self.scorable_to_group.get(&scorable_id)
            .ok_or(StoreError::ScorableNotFound(scorable_id))?;
        let group = self.scores.get_mut(group_id)
            .ok_or(StoreError::GroupNotFound(*group_id))?;
        group.scorables.get_mut(&scorable_id)
            .ok_or(StoreError::ScorableNotFound(scorable_id))?
            .scores.remove(id)
            .ok_or(StoreError::ScoreNotFound(*id))
            .map(|_| ())
    }

    pub fn groups(&self) -> Result<Vec<crate::store_interface::Group>,StoreError> {
        let mut groups: Vec<_> = self.scores
            .iter()
            .map(|(id,group)| store_interface::Group {
                id: *id,
                name: group.name.to_owned()
            })
            .collect();
        groups.sort();
        Ok(groups)
    }
    pub fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<crate::store_interface::Scorable>,StoreError> {
        let mut scorables: Vec<_> = self.scores.get(&group_id)
            .ok_or(StoreError::GroupNotFound(*group_id))?
            .iter_scorables()
            .map(|(id,scorable)| store_interface::Scorable {
                id: id,
                name: scorable.name.to_owned()
            })
            .collect();
        scorables.sort();
        Ok(scorables)
    }
    pub fn get_scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<crate::store_interface::Score>,StoreError> {
        let group_id = self.scorable_to_group.get(&scorable_id)
            .ok_or(StoreError::ScorableNotFound(*scorable_id))?;
        let group = self.scores.get(group_id)
            .ok_or(StoreError::GroupNotFound(*group_id))?;
        let mut scores: Vec<_> = group.scorables.get(scorable_id)
            .ok_or(StoreError::ScorableNotFound(*scorable_id))?
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

    fn last_changed(&self) -> DateTime<Utc> {
        self.last_changed
    }
    fn update_last_changed(&mut self) {
        self.last_changed = Utc::now();
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

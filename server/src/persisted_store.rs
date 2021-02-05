use std::path::PathBuf;
use serde::de::value;

use crate::events::{ EventHandler, Event };
use crate::memory_store::{ MemoryStore, MemoryStoreError };
use crate::store_interface::{ Store, GroupId, ScoreId, ScorableId, Group, Score, HashedPassword };

/// An in-memory data store.
pub struct PersistedStore {
    /// Read and write events to persist
    events: EventHandler,
    /// In-memory data derived from events:
    memory_store: MemoryStore
}

impl PersistedStore {
    /// Load in our data from a file
    pub async fn load(file_path: PathBuf) -> anyhow::Result<PersistedStore> {
        let events = EventHandler::new(file_path);
        let memory_store = MemoryStore::from_events(&events).await?;
        Ok(PersistedStore { events, memory_store })
    }
}

// This store uses a memory store for most reads and writes, but also writes to
// an event log to achieve persistence where necessary.
#[async_trait::async_trait]
impl Store for PersistedStore {
    type Error = MemoryStoreError;

    async fn upsert_user(&mut self, username: String, hashed_password: HashedPassword) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertUser {
            username: username.clone(),
            hashed_password: hashed_password.clone()
        }).await;
        self.memory_store.upsert_user(username, hashed_password)
    }
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,Self::Error> {
        self.memory_store.check_user(username, password)
    }
    async fn delete_user(&mut self, username: &str) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteUser {
            username: username.to_owned()
        }).await;
        self.memory_store.delete_user(username)
    }

    async fn upsert_group(&mut self, id: GroupId, name: String) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertGroup {
            id,
            name: name.clone()
        }).await;
        self.memory_store.upsert_group(id, name)
    }
    async fn delete_group(&mut self, id: &GroupId) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteGroup {
            id: *id,
        }).await;
        self.memory_store.delete_group(id)
    }

    async fn upsert_scorable(&mut self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertScorable {
            id,
            group_id,
            name: name.clone()
        }).await;
        self.memory_store.upsert_scorable(id, group_id, name)
    }
    async fn delete_scorable(&mut self, id: &ScorableId) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteScorable {
            id: *id
        }).await;
        self.memory_store.delete_scorable(id)
    }

    async fn upsert_score(&mut self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: chrono::DateTime<chrono::Utc>) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertScore {
            date,
            id,
            value,
            username: username.clone(),
            scorable_id
        }).await;
        self.memory_store.upsert_score(id, scorable_id, username, value, date)
    }
    async fn delete_score(&mut self, id: &ScoreId) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteScore {
            id: *id
        }).await;
        self.memory_store.delete_score(id)
    }

    async fn groups(&self) -> Result<Vec<Group>,Self::Error> {
        self.memory_store.groups()
    }
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<crate::store_interface::Scorable>,Self::Error> {
        self.memory_store.scorables_in_group(group_id)
    }
    async fn get_scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<Score>,Self::Error> {
        self.memory_store.get_scores(scorable_id, limit)
    }
}

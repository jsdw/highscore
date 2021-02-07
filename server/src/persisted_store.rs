use std::path::PathBuf;
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

pub type PersistedStoreError = MemoryStoreError;

impl PersistedStore {
    /// Load in our data from a file
    pub async fn load(file_path: PathBuf) -> anyhow::Result<PersistedStore> {
        let events = EventHandler::new(file_path);
        let memory_store = MemoryStore::from_events(&events).await?;
        Ok(PersistedStore { events, memory_store })
    }

    /// Force anything in-memory to be flushed to disk immediately.
    pub async fn flush_to_disk(&self) -> anyhow::Result<()> {
        self.events.flush_to_disk().await
    }
}

// This store uses a memory store for most reads and writes, but also writes to
// an event log to achieve persistence where necessary.
#[async_trait::async_trait]
impl Store for PersistedStore {
    type Error = PersistedStoreError;

    async fn upsert_user(&self, username: String, hashed_password: HashedPassword) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertUser {
            username: username.clone(),
            hashed_password: hashed_password.clone()
        }).await;
        self.memory_store.upsert_user(username, hashed_password).await
    }
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,Self::Error> {
        self.memory_store.check_user(username, password).await
    }
    async fn delete_user(&self, username: &str) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteUser {
            username: username.to_owned()
        }).await;
        self.memory_store.delete_user(username).await
    }

    async fn upsert_group(&self, id: GroupId, name: String) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertGroup {
            id,
            name: name.clone()
        }).await;
        self.memory_store.upsert_group(id, name).await
    }
    async fn delete_group(&self, id: &GroupId) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteGroup {
            id: *id,
        }).await;
        self.memory_store.delete_group(id).await
    }

    async fn upsert_scorable(&self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertScorable {
            id,
            group_id,
            name: name.clone()
        }).await;
        self.memory_store.upsert_scorable(id, group_id, name).await
    }
    async fn delete_scorable(&self, id: &ScorableId) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteScorable {
            id: *id
        }).await;
        self.memory_store.delete_scorable(id).await
    }

    async fn upsert_score(&self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: chrono::DateTime<chrono::Utc>) -> Result<(),Self::Error> {
        self.events.push(Event::UpsertScore {
            date,
            id,
            value,
            username: username.clone(),
            scorable_id
        }).await;
        self.memory_store.upsert_score(id, scorable_id, username, value, date).await
    }
    async fn delete_score(&self, id: &ScoreId) -> Result<(),Self::Error> {
        self.events.push(Event::DeleteScore {
            id: *id
        }).await;
        self.memory_store.delete_score(id).await
    }

    async fn groups(&self) -> Result<Vec<Group>,Self::Error> {
        self.memory_store.groups().await
    }
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<crate::store_interface::Scorable>,Self::Error> {
        self.memory_store.scorables_in_group(group_id).await
    }
    async fn scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<Score>,Self::Error> {
        self.memory_store.scores(scorable_id, limit).await
    }
}

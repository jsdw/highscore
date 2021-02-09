use std::path::PathBuf;
use crate::events::{ EventHandler, Event };
use crate::memory_store::{ MemoryStore };
use crate::store_interface::{ Store, StoreError, GroupId, ScoreId, ScorableId, Group, Score, Scorable, HashedPassword };

/// This combines an in-memory `Store` implementation with eventual
/// persistence in the form of append-only event logs.
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

    /// Force anything in-memory to be flushed to disk immediately.
    pub async fn flush_to_disk(&self) -> anyhow::Result<()> {
        self.events.flush_to_disk().await
    }
}

// This implementation uses a memory store for most reads and writes, but also writes to
// an event log ocasionally to achieve eventual persistence.
//
// Writes to the event log only happen once the call to the memory_store has
// succeeded, to avoid writing naff data to the event log and lean on memory_store
// to check that inputs are sensible.
#[async_trait::async_trait]
impl Store for PersistedStore {
    async fn last_changed(&self) -> chrono::DateTime<chrono::Utc> {
        self.memory_store.last_changed().await
    }

    async fn upsert_user(&self, username: String, hashed_password: HashedPassword) -> Result<(),StoreError> {
        let res = self.memory_store.upsert_user(username.clone(), hashed_password.clone()).await?;
        self.events.push(Event::UpsertUser {
            username: username,
            hashed_password: hashed_password
        }).await;
        Ok(res)
    }
    async fn check_user(&self, username: &str, password: &str) -> Result<bool,StoreError> {
        self.memory_store.check_user(username, password).await
    }
    async fn delete_user(&self, username: &str) -> Result<(),StoreError> {
        let res = self.memory_store.delete_user(username).await?;
        self.events.push(Event::DeleteUser {
            username: username.to_owned()
        }).await;
        Ok(res)
    }

    async fn upsert_group(&self, id: GroupId, name: String) -> Result<(),StoreError> {
        self.memory_store.upsert_group(id, name.clone()).await?;
        self.events.push(Event::UpsertGroup {
            id,
            name
        }).await;
        Ok(())
    }
    async fn delete_group(&self, id: &GroupId) -> Result<(),StoreError> {
        let res = self.memory_store.delete_group(id).await?;
        self.events.push(Event::DeleteGroup {
            id: *id,
        }).await;
        Ok(res)
    }
    async fn get_group(&self, id: &GroupId) -> Result<Group,StoreError> {
        self.memory_store.get_group(id).await
    }

    async fn upsert_scorable(&self, id: ScorableId, group_id: GroupId, name: String) -> Result<(),StoreError> {
        self.memory_store.upsert_scorable(id, group_id, name.clone()).await?;
        self.events.push(Event::UpsertScorable {
            id,
            group_id,
            name
        }).await;
        Ok(())
    }
    async fn delete_scorable(&self, id: &ScorableId) -> Result<(),StoreError> {
        let res = self.memory_store.delete_scorable(id).await?;
        self.events.push(Event::DeleteScorable {
            id: *id
        }).await;
        Ok(res)
    }
    async fn get_scorable(&self, id: &ScorableId) -> Result<Scorable,StoreError> {
        self.memory_store.get_scorable(id).await
    }

    async fn upsert_score(&self, id: ScoreId, scorable_id: ScorableId, username: String, value: i64, date: chrono::DateTime<chrono::Utc>) -> Result<(),StoreError> {
        self.memory_store.upsert_score(id, scorable_id, username.clone(), value, date).await?;
        self.events.push(Event::UpsertScore {
            date,
            id,
            value,
            username: username,
            scorable_id
        }).await;
        Ok(())
    }
    async fn delete_score(&self, id: &ScoreId) -> Result<(),StoreError> {
        let res = self.memory_store.delete_score(id).await?;
        self.events.push(Event::DeleteScore {
            id: *id
        }).await;
        Ok(res)
    }

    async fn groups(&self) -> Result<Vec<Group>,StoreError> {
        self.memory_store.groups().await
    }
    async fn scorables_in_group(&self, group_id: &GroupId) -> Result<Vec<Scorable>,StoreError> {
        self.memory_store.scorables_in_group(group_id).await
    }
    async fn scores(&self, scorable_id: &ScorableId, limit: Option<usize>) -> Result<Vec<Score>,StoreError> {
        self.memory_store.scores(scorable_id, limit).await
    }
}

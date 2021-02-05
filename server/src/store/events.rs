use chrono::prelude::{ DateTime, Utc };
use uuid::Uuid;
use serde::{ Serialize, Deserialize };
use std::sync::{ Arc };
use std::time::Duration;
use std::path::PathBuf;
use tokio::{io::AsyncWriteExt, sync::Mutex};
use futures::stream::Stream;
use std::marker::Unpin;

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct GroupId(Uuid);

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct ScorableId(Uuid);

#[derive(Serialize,Deserialize,Hash,PartialEq,Eq,Debug,Clone,Copy)]
pub struct ScoreId(Uuid);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    /// Add/update a user in the system
    AddUser { username: String, hashed_password: String },
    /// Delete user from the system
    DeleteUser { username: String },

    /// Add/update a group for scores to live under
    AddGroup { id: GroupId, name: String },
    /// Delete a group (and everything in it)
    DeleteGroup { id: GroupId },

    /// Add thing to score (and all scores against it)
    AddScorable { id: ScorableId, group_id: GroupId, name: String },
    /// Remove a thing to score (and all scores against it)
    DeleteScorable { id: ScorableId },

    /// Add a score to a group at a date
    AddScore { id: ScoreId, scorable_id: ScorableId, value: i64, date: DateTime<Utc> },
    /// Remove a score from a group
    DeleteScore { id: ScoreId }
}

struct Events {
    file_path: PathBuf,
    in_memory: Arc<Mutex<Vec<Event>>>
}

impl Events {

    fn new(file_path: PathBuf) -> Events {
        Events {
            file_path,
            in_memory: Arc::new(Mutex::new(Vec::new()))
        }
    }

    async fn is_empty(&self) -> bool {
        self.in_memory.lock().await.is_empty()
    }

    async fn push(&self, ev: Event) {
        self.in_memory.lock().await.push(ev)
    }

    async fn read_from_disk(&self) -> anyhow::Result<impl Stream<Item = Result<Event,anyhow::Error>> + Unpin + Send + Sync + 'static> {
        use tokio::io::AsyncBufReadExt;
        let file = tokio::fs::File::open(&self.file_path).await?;
        let buf = tokio::io::BufReader::new(file);

        // return a stream of events, ignoring any lines in the file which
        // aren't valid events for whatever reason. Avoid allocations by reusing the
        // same string buffer over and over. Return an error if reading from the file errors.
        Ok(Box::pin(futures::stream::try_unfold((buf,String::new()), |(mut buf, mut line)| async move {
            let event = loop {
                line.clear();
                buf.read_line(&mut line).await?;
                let event = serde_json::from_str(&line);
                if let Ok(ev) = event { break ev }
            };
            Ok(Some((event,(buf,line))))
        })))
    }

    async fn flush_to_disk(&self) -> anyhow::Result<()> {
        let mut events = self.in_memory.lock().await;
        if events.is_empty() {
            return Ok(())
        }
        let mut file = tokio::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.file_path).await?;
        for event in &*events {
            let event_json = serde_json::to_vec(&event)?;
            file.write_all(b"\n").await?;
            file.write(&event_json).await?;
        }
        file.flush().await?;
        *events = Vec::new();
        Ok(())
    }

}

pub struct EventHandler {
    in_memory_events: Arc<Events>
}

impl EventHandler {

    /// Create a new event handler by providing a path o ndisk to where
    /// events will be persisted. This must run in a `tokio` context.
    pub fn new(file_path: std::path::PathBuf) -> EventHandler {

        let in_memory_events = Arc::new(Events::new(file_path));
        let in_memory_events2 = Arc::clone(&in_memory_events);

        // Periodically flush events to disk:
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(30)).await;
                in_memory_events2.flush_to_disk().await;
                // TODO: Log errors flushng to disk.
            }
        });

        EventHandler {
            in_memory_events
        }
    }

    /// Push a new event to the stream
    pub async fn push(&self, ev: Event) {
        self.in_memory_events.push(ev).await
    }

    /// Read events from disk
    pub async fn read_from_disk(&self) -> anyhow::Result<impl Stream<Item = Result<Event,anyhow::Error>> + Unpin> {
        self.in_memory_events.read_from_disk().await
    }

}
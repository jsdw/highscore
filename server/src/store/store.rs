use std::path::PathBuf;
use super::events::{ EventHandler };
use super::data::Data;

/// An in-memory data store.
pub struct Store {
    /// Read and write events to persist
    events: EventHandler,
    /// In-memory data derived from events:
    data: Data
}

impl Store {
    /// Load in our data from a file
    pub async fn load(file_path: PathBuf) -> anyhow::Result<Store> {
        let events = EventHandler::new(file_path);
        let data = Data::from_events(&events).await?;
        Ok(Store { events, data })
    }

}



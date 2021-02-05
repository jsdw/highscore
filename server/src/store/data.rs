use std::collections::HashMap;
use chrono::prelude::{ DateTime, Utc };
use super::events::{ Event, EventHandler, GroupId, ScorableId, ScoreId };

pub struct Data {
    /// Users (mapping of username to password)
    users: HashMap<String,String>,
    /// Groups of scorables that themselves have scores on
    scores: HashMap<GroupId, Group>
}

impl Data {
    /// Load data in from persisted events.
    pub async fn from_events(events: &EventHandler) -> anyhow::Result<Data> {
        use futures::stream::StreamExt;
        let mut data = Data {
            users: HashMap::new(),
            scores: HashMap::new()
        };

        // Cache some details as we go:
        let mut scorable_to_group = HashMap::new();
        let mut score_to_scorable = HashMap::new();

        let mut event_stream = events.read_from_disk().await?;
        while let Some(event) = event_stream.next().await {
            let event = event?;
            // TODO: Log when events can't complete successfully (mainly
            // because something doesn't exist).
            match event {
                Event::AddUser { username, hashed_password } => {
                    data.users.insert(username, hashed_password);
                }
                Event::DeleteUser { username } => {
                    data.users.remove(&username);
                }
                Event::AddGroup { id, name } => {
                    data.scores.insert(id, Group::new(name));
                }
                Event::DeleteGroup { id } => {
                    data.scores.remove(&id);
                }
                Event::AddScorable { id, group_id, name } => {
                    if let Some(group) = data.scores.get_mut(&group_id) {
                        group.scorables.insert(id, Scorable::new(name));
                        scorable_to_group.insert(id, group_id);
                    }
                }
                Event::DeleteScorable { id } => {
                    let group_id = match scorable_to_group.remove(&id) {
                        Some(group_id) => group_id,
                        None => continue
                    };
                    if let Some(group) = data.scores.get_mut(&group_id) {
                        group.scorables.remove(&id);
                    }
                }
                Event::AddScore { id, scorable_id, value, date } => {
                    let group_id = match scorable_to_group.get(&scorable_id) {
                        Some(group_id) => group_id,
                        None => continue
                    };
                    let group = match data.scores.get_mut(&group_id) {
                        Some(group) => group,
                        None => continue
                    };
                    if let Some(scorable) = group.scorables.get_mut(&scorable_id) {
                        scorable.scores.insert(id, Score { value, date });
                        score_to_scorable.insert(id, scorable_id);
                    }
                }
                Event::DeleteScore { id } => {
                    let scorable_id = match score_to_scorable.remove(&id) {
                        Some(scorable_id) => scorable_id,
                        None => continue
                    };
                    let group_id = match scorable_to_group.get(&scorable_id) {
                        Some(group_id) => group_id,
                        None => continue
                    };
                    let group = match data.scores.get_mut(&group_id) {
                        Some(group) => group,
                        None => continue
                    };
                    if let Some(scorable) = group.scorables.get_mut(&scorable_id) {
                        scorable.scores.remove(&id);
                    }
                }
            }
        }
        Ok(data)
    }
}

struct Group {
    name: String,
    scorables: HashMap<ScorableId, Scorable>
}

impl Group {
    fn new(name: String) -> Group {
        Group { name, scorables: HashMap::new() }
    }
}

struct Scorable {
    name: String,
    scores: HashMap<ScoreId, Score>
}

impl Scorable {
    fn new(name: String) -> Scorable {
        Scorable { name, scores: HashMap::new() }
    }
}

struct Score {
    value: i64,
    date: DateTime<Utc>
}
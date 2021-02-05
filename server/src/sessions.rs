use std::{collections::HashMap, fmt::Display};
use uuid::Uuid;
use std::str::FromStr;

pub struct Sessions {
    /// Map of SessionId to username
    sessions: HashMap<SessionId,String>
}

impl Sessions {

    pub fn new() -> Sessions {
        Sessions {
            sessions: HashMap::new()
        }
    }

    pub fn get(&self, id: &SessionId) -> Option<&str> {
        self.sessions.get(id).map(|s| &**s)
    }

    pub fn create(&mut self, username: String) -> SessionId {
        let new_id = SessionId::new();
        self.sessions.insert(new_id.clone(), username);
        new_id
    }

    pub fn remove(&mut self, id: &SessionId) {
        self.sessions.remove(id);
    }

}

#[derive(Clone,Hash,PartialEq,Eq)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> SessionId {
        SessionId(Uuid::new_v4())
    }
}

impl FromStr for SessionId {
    type Err = <Uuid as FromStr>::Err;
    fn from_str(s: &str) -> Result<SessionId,Self::Err> {
        Ok(SessionId(Uuid::from_str(s)?))
    }
}

impl Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::resources::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub id: String,
    pub unavailable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub id: String,
    pub flags: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ready {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<Guild>,
    pub session_id: String,
    pub session_type: String,
    pub application: Application,
    pub shard: Option<[u32; 2]>,
    #[serde(rename = "_trace")]
    pub trace: Vec<String>,
}

impl Ready {
    pub fn from_value(value: &Value) -> Option<Self> {
        match serde_json::from_value(value.clone()) {
            Ok(ready) => Some(ready),
            Err(err) => {
                eprintln!("Error deserializing Ready data: {}", err);
                None
            }
        }
    }
}
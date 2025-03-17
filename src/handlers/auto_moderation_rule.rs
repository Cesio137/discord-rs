use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionMetadata {
    pub channel_id: Option<String>,
    pub duration_seconds: Option<u32>,
    pub custom_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub type_: u8,
    pub metadata: Option<ActionMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerMetadata {
    pub keyword_filter: Option<Vec<String>>,
    pub presets: Option<Vec<u8>>,
    pub allow_list: Option<Vec<String>>,
    pub mention_total_limit: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoModerationRule {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    pub creator_id: String,
    pub event_type: u8,
    pub trigger_type: u8,
    pub trigger_metadata: TriggerMetadata,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub exempt_roles: Vec<String>,
    pub exempt_channels: Vec<String>,
}

impl AutoModerationRule {
    pub fn from_value(value: &Value) -> Option<Self> {
        match serde_json::from_value(value.clone()) {
            Ok(rule) => Some(rule),
            Err(err) => {
                eprintln!("Error deserializing AutoModerationRule data: {}", err);
                None
            }
        }
    }
}
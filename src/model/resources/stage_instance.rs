use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PrivacyLevel {
    PUBLIC= 1,
    GUILD_ONLY = 2
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StageInstance {
    pub id: String,
    pub guild_id: String,
    pub channel_id: String,
    pub topic: String,
    pub privacy_level: PrivacyLevel,
    pub discoverable_disabled: bool,
    pub guild_scheduled_event_id: Option<String>,
}
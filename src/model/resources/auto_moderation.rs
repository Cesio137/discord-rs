use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum EventType {
    MESSAGE_SEND = 1,
    MEMBER_UPDATE = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum TriggerType {
    KEYWORD = 1,
    SPAM = 3,
    KEYWORD_PRESET = 4,
    MENTION_SPAM = 5,
    MEMBER_PROFILE = 6
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ActionType {
    BLOCK_MESSAGE = 1,
    SEND_ALERT_MESSAGE = 2,
    TIMEOUT = 3,
    BLOCK_MEMBER_INTERACTION = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum KeywordPresetTypes {
    PROFANITY = 1,
    SEXUAL_CONTENT = 2,
    SLURS = 3,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutoModerationRule {
    pub id: String,
    pub guild_id: String,
    pub name: String,
    pub creator_id: String,
    pub event_type: EventType,
    pub trigger_type: TriggerType,
    pub trigger_metadata: TriggerMetadata,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub exempt_roles: Vec<String>,
    pub exempt_channels: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    pub type_: ActionType,
    pub metadata: Option<ActionMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActionMetadata {
    pub channel_id: Option<String>,
    pub duration_seconds: Option<u32>,
    pub custom_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TriggerMetadata {
    pub keyword_filter: Vec<String>,
    pub regex_patterns: Vec<String>,
    pub presets: Vec<KeywordPresetTypes>,
    pub allow_list: Option<Vec<String>>,
    pub mention_total_limit: Option<u32>,
}
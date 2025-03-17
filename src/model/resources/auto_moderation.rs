use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventType {
    MESSAGE_SEND = 1,
    MEMBER_UPDATE = 2,
}

impl DiscordTypes for EventType {
    fn value(&self) -> u8 {
        match self {
            EventType::MESSAGE_SEND => 1,
            EventType::MEMBER_UPDATE => 2,
        }
    }

    fn from(value: u8) -> Self {
        match value {
            1 => EventType::MESSAGE_SEND,
            2 => EventType::MEMBER_UPDATE,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TriggerType {
    KEYWORD = 1,
    SPAM = 3,
    KEYWORD_PRESET = 4,
    MENTION_SPAM = 5,
    MEMBER_PROFILE = 6
}

impl DiscordTypes for TriggerType {
    fn from(value: u8) -> Self {
        match value {
            1 => TriggerType::KEYWORD,
            3 => TriggerType::SPAM,
            4 => TriggerType::KEYWORD_PRESET,
            5 => TriggerType::MENTION_SPAM,
            6 => TriggerType::MEMBER_PROFILE,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            TriggerType::KEYWORD => 1,
            TriggerType::SPAM => 3,
            TriggerType::KEYWORD_PRESET => 4,
            TriggerType::MENTION_SPAM => 5,
            TriggerType::MEMBER_PROFILE => 6,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionType {
    BLOCK_MESSAGE = 1,
    SEND_ALERT_MESSAGE = 2,
    TIMEOUT = 3,
    BLOCK_MEMBER_INTERACTION = 4,
}

impl DiscordTypes for ActionType {
    fn from(value: u8) -> Self {
        match value {
            1 => ActionType::BLOCK_MESSAGE,
            2 => ActionType::SEND_ALERT_MESSAGE,
            3 => ActionType::TIMEOUT,
            4 => ActionType::BLOCK_MEMBER_INTERACTION,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            ActionType::BLOCK_MESSAGE => 1,
            ActionType::SEND_ALERT_MESSAGE => 2,
            ActionType::TIMEOUT => 3,
            ActionType::BLOCK_MEMBER_INTERACTION => 4,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum KeywordPresetTypes {
    PROFANITY = 1,
    SEXUAL_CONTENT = 2,
    SLURS = 3,
}

impl DiscordTypes for KeywordPresetTypes {
    fn from(value: u8) -> Self {
        match value {
            1 => KeywordPresetTypes::PROFANITY,
            2 => KeywordPresetTypes::SLURS,
            3 => KeywordPresetTypes::SLURS,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            KeywordPresetTypes::PROFANITY => 1,
            KeywordPresetTypes::SEXUAL_CONTENT => 2,
            KeywordPresetTypes::SLURS => 3,
        }
    }
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
use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PrivacyLevel {
    PUBLIC= 1,
    GUILD_ONLY = 2
}

impl DiscordTypes for PrivacyLevel {
    fn from(value: u8) -> Self {
        match value { 
            1 => PrivacyLevel::PUBLIC,
            2 => PrivacyLevel::GUILD_ONLY,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self { 
            PrivacyLevel::PUBLIC => 1,
            PrivacyLevel::GUILD_ONLY => 2,
        }
    }
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
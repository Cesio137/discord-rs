use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GuildScheduledEventPrivacyLevel {
    GuildOnly = 2,
}

impl DiscordTypes for GuildScheduledEventPrivacyLevel {
    fn from(value: u8) -> Self {
        match value {
            2 => GuildScheduledEventPrivacyLevel::GuildOnly,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            GuildScheduledEventPrivacyLevel::GuildOnly => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GuildScheduledEventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

impl DiscordTypes for GuildScheduledEventStatus {
    fn from(value: u8) -> Self {
        match value {
            1 => GuildScheduledEventStatus::Scheduled,
            2 => GuildScheduledEventStatus::Active,
            3 => GuildScheduledEventStatus::Completed,
            4 => GuildScheduledEventStatus::Canceled,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            GuildScheduledEventStatus::Scheduled => 1,
            GuildScheduledEventStatus::Active => 2,
            GuildScheduledEventStatus::Completed => 3,
            GuildScheduledEventStatus::Canceled => 4,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GuildScheduledEventEntityType {
    StageInstance = 1,
    Voice = 2,
    External = 3,
}

impl DiscordTypes for GuildScheduledEventEntityType {
    fn from(value: u8) -> Self {
        match value {
            1 => GuildScheduledEventEntityType::StageInstance,
            2 => GuildScheduledEventEntityType::Voice,
            3 => GuildScheduledEventEntityType::External,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            GuildScheduledEventEntityType::StageInstance => 1,
            GuildScheduledEventEntityType::Voice => 2,
            GuildScheduledEventEntityType::External => 3,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildScheduledEvent {
    pub id: String,
    pub guild_id: String,
    pub channel_id: Option<String>,
    pub creator_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub scheduled_start_time: String,
    pub scheduled_end_time: Option<String>,
    pub privacy_level: GuildScheduledEventPrivacyLevel,
    pub status: GuildScheduledEventStatus,
    #[serde(rename = "entity_type")]
    pub entity_type: GuildScheduledEventEntityType,
    pub entity_id: Option<String>,
    pub entity_metadata: Option<GuildScheduledEventEntityMetadata>,
    pub creator: Option<super::super::user::User>,
    pub user_count: Option<i32>,
    pub image: Option<String>,
    pub recurrence_rule: Option<GuildScheduledEventRecurrenceRule>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildScheduledEventEntityMetadata {
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildScheduledEventRecurrenceRule {
    pub frequency: String,
    pub interval: i32,
    pub count: Option<i32>,
    pub until: Option<String>,
    pub by_day: Option<Vec<String>>,
    pub by_month_day: Option<Vec<i32>>,
    pub by_year_day: Option<Vec<i32>>,
    pub by_week_no: Option<Vec<i32>>,
    pub by_month: Option<Vec<i32>>,
    pub by_set_pos: Option<Vec<i32>>,
    pub week_start: Option<String>,
}
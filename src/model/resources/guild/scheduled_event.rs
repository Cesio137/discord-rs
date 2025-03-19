use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GuildScheduledEventPrivacyLevel {
    GuildOnly = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GuildScheduledEventStatus {
    Scheduled = 1,
    Active = 2,
    Completed = 3,
    Canceled = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GuildScheduledEventEntityType {
    StageInstance = 1,
    Voice = 2,
    External = 3,
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
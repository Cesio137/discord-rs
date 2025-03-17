use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Overwrite {
    pub id: String,
    pub type_: u8,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelCreate {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: u8,
    pub guild_id: Option<String>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u32>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<serde_json::Value>>, // You might want to create a User struct for this later
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u8>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<serde_json::Value>, // You might want to create a ThreadMetadata struct for this later
    pub member: Option<serde_json::Value>, // You might want to create a ThreadMember struct for this later
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u32>,
    pub total_message_sent: Option<u32>,
}

impl ChannelCreate {
    pub fn from_value(value: &Value) -> Option<Self> {
        match serde_json::from_value(value.clone()) {
            Ok(channel) => Some(channel),
            Err(err) => {
                eprintln!("Error deserializing ChannelCreate data: {}", err);
                None
            }
        }
    }
}
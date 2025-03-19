use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum WebhookTypes {
    INCOMING = 1,
    CHANNEL_FOLLOWER = 2,
    APPLICATION = 3,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Webhook {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: WebhookTypes,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    pub user: Option<super::user::User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub token: Option<String>,
    pub application_id: Option<String>,
    pub source_guild: Option<super::guild::Guild>,
    pub source_channel: Option<super::channel::Channel>,
    pub url: Option<String>,
}
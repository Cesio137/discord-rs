use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WebhookTypes {
    INCOMING = 1,
    CHANNEL_FOLLOWER = 2,
    APPLICATION = 3,
}

impl DiscordTypes for WebhookTypes {
    fn from(value: u8) -> Self {
        match value { 
            1 => WebhookTypes::INCOMING,
            2 => WebhookTypes::CHANNEL_FOLLOWER,
            3 => WebhookTypes::APPLICATION,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self { 
            WebhookTypes::INCOMING => 1,
            WebhookTypes::CHANNEL_FOLLOWER => 2,
            WebhookTypes::APPLICATION => 3,
        }
    }
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
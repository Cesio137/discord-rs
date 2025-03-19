pub mod application_commands;
pub mod message_components;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::application::IntegrationType;
use crate::model::resources::channel::Channel;
use crate::model::resources::entitlement::Entitlement;
use crate::model::resources::guild::{Guild, GuildMember};
use crate::model::resources::message::Message;
use crate::model::resources::user::User;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum InteractionContextType {
    GUILD = 0,
    BOT_DM = 1,
    PRIVATE_CHANNEL = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum EntryPointCommandHandlerType {
    APP_HANDLER = 1,
    DISCORD_LAUNCH_ACTIVITY = 2,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Interaction {
    pub id: String,
    pub application_id: String,
    #[serde(rename = "type")]
    pub format_type: i32,
    pub data: Option<InteractionType>,
    pub guild: Option<Guild>,
    pub guild_id: Option<String>,
    pub channel: Option<Channel>,
    pub channel_id: Option<String>,
    pub member: Option<GuildMember>,
    pub user: Option<User>,
    pub token: String,
    pub version: i32,
    pub message: Option<Message>,
    pub app_permissions: Option<String>,
    pub locale: Option<String>,
    pub guild_locale: Option<String>,
    pub entitlements: Option<Vec<Entitlement>>,
    pub authorizing_integration_owners: Option<HashMap<String, IntegrationType>>,
    pub context: Option<InteractionContextType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageInteraction {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: InteractionType,
    pub name: String,
    pub user: User,
    pub member: Option<GuildMember>,
}
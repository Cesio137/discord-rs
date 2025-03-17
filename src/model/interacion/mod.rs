pub mod application_commands;
pub mod message_components;

use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
use crate::model::resources::guild::GuildMember;
use crate::model::resources::user::User;

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InteractionType {
    Ping = 1,
    ApplicationCommand = 2,
    MessageComponent = 3,
    ApplicationCommandAutocomplete = 4,
    ModalSubmit = 5,
}

impl DiscordTypes for InteractionType {
    fn from(value: u8) -> Self {
        match value {
            1 => InteractionType::Ping,
            2 => InteractionType::ApplicationCommand,
            3 => InteractionType::MessageComponent,
            4 => InteractionType::ApplicationCommandAutocomplete,
            5 => InteractionType::ModalSubmit,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            InteractionType::Ping => 1,
            InteractionType::ApplicationCommand => 2,
            InteractionType::MessageComponent => 3,
            InteractionType::ApplicationCommandAutocomplete => 4,
            InteractionType::ModalSubmit => 5,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractionContextType {
    GUILD = 0,
    BOT_DM = 1,
    PRIVATE_CHANNEL = 2,
}

impl DiscordTypes for InteractionContextType {
    fn from(value: u8) -> Self {
        match value {
            0 => InteractionContextType::GUILD,
            1 => InteractionContextType::BOT_DM,
            2 => InteractionContextType::PRIVATE_CHANNEL,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            InteractionContextType::GUILD => 0,
            InteractionContextType::BOT_DM => 1,
            InteractionContextType::PRIVATE_CHANNEL => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntryPointCommandHandlerType {
    APP_HANDLER = 1,
    DISCORD_LAUNCH_ACTIVITY = 2,
}

impl DiscordTypes for EntryPointCommandHandlerType {
    fn from(value: u8) -> Self {
        match value {
            1 => EntryPointCommandHandlerType::APP_HANDLER,
            2 => EntryPointCommandHandlerType::DISCORD_LAUNCH_ACTIVITY,
            _ => EntryPointCommandHandlerType::APP_HANDLER,
        }
    }

    fn value(&self) -> u8 {
        match self {
            EntryPointCommandHandlerType::APP_HANDLER => 1,
            EntryPointCommandHandlerType::DISCORD_LAUNCH_ACTIVITY => 2,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageInteraction {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: InteractionType,
    pub name: String,
    pub user: User,
    pub member: Option<GuildMember>,
}
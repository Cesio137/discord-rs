use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
use crate::model::resources::application::IntegrationType;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i32),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AplicationCommandType {
    CHAT_INPUT = 1,
    USER = 2,
    MESSAGE = 3,
    PRIMARY_ENTRY_POINT = 4,
}

impl DiscordTypes for AplicationCommandType {
    fn from(value: u8) -> Self {
        match value {
            1 => AplicationCommandType::CHAT_INPUT,
            2 => AplicationCommandType::USER,
            3 => AplicationCommandType::MESSAGE,
            4 => AplicationCommandType::PRIMARY_ENTRY_POINT,
            _ => unreachable!(),
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            AplicationCommandType::CHAT_INPUT => 1,
            AplicationCommandType::USER => 2,
            AplicationCommandType::MESSAGE => 3,
            AplicationCommandType::PRIMARY_ENTRY_POINT => 4,
        }
    }
}


/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommand {
    pub id: String,
    pub type_: Option<AplicationCommandType>,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub name: String,
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    pub options: Option<Vec<crate::interactions::application_commands::ApplicationCommandOption>>,
    pub default_member_permissions: Option<String>,
    pub dm_permission: Option<bool>,
    pub default_permission: Option<bool>,
    pub nsfw: Option<bool>,
    pub integration_types: Option<Vec<IntegrationType>>,
    pub contexts: Option<Vec<super::InteractionContextType>>,
    pub version: String,
    pub handler: Option<Vec<super::EntryPointCommandHandlerType>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub type_: u8,
    pub name: String,
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    pub required: Option<bool>,
    pub choices: Option<Vec<crate::interactions::application_commands::ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<crate::interactions::application_commands::ApplicationCommandOption>>,
    pub channel_types: Option<Vec<u8>>,
    pub min_value: Option<i32>,
    pub max_value: Option<i32>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub autocomplete: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOptionChoice {
    pub name: String,
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    pub value: crate::interactions::application_commands::ApplicationCommandOptionChoiceValue,
}
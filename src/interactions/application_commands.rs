use serde::{Deserialize, Serialize};
use crate::resources::types::application::ApplicationIntegrationType;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommand {
    pub id: String,
    pub type_: Option<super::types::application_command::TypeAplicationCommand>,
    pub application_id: String,
    pub guild_id: Option<String>,
    pub name: String,
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub default_member_permissions: Option<String>,
    pub dm_permission: Option<bool>,
    pub default_permission: Option<bool>,
    pub nsfw: Option<bool>,
    pub integration_types: Option<Vec<ApplicationIntegrationType>>,
    pub contexts: Option<Vec<super::types::TypeInteractionContext>>,
    pub version: String,
    pub handler: Option<Vec<super::types::TypeEntryPointCommandHandler>>,
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
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
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
    pub value: ApplicationCommandOptionChoiceValue,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i32),
    Number(f64),
}
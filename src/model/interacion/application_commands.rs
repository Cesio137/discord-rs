use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::application::IntegrationType;
use crate::model::resources::channel::ChannelType;
/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum AplicationCommandType {
    CHAT_INPUT = 1,
    USER = 2,
    MESSAGE = 3,
    PRIMARY_ENTRY_POINT = 4,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
    Mentionable = 9,
    Number = 10,
    Attachment = 11,
}


/*UNTAGGED*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum ApplicationCommandOptionChoiceValue {
    String(String),
    Integer(i32),
    Number(f64),
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommand {
    pub id: String,
    pub format_type: Option<AplicationCommandType>,
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
    pub integration_types: Option<Vec<IntegrationType>>,
    pub contexts: Option<Vec<super::InteractionContextType>>,
    pub version: String,
    pub handler: Option<Vec<super::EntryPointCommandHandlerType>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    pub format_type: ApplicationCommandOptionType,
    pub name: String,
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
    pub required: Option<bool>,
    pub choices: Option<Vec<ApplicationCommandOptionChoice>>,
    pub options: Option<Vec<ApplicationCommandOption>>,
    pub channel_types: Option<Vec<ChannelType>>,
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
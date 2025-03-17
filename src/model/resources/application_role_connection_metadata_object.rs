use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ConnectionMetadataType {
    INTEGER_LESS_THAN_OR_EQUAL = 1,
    INTEGER_GREATER_THAN_OR_EQUAL = 2,
    INTEGER_EQUAL = 3,
    INTEGER_NOT_EQUAL = 4,
    DATETIME_LESS_THAN_OR_EQUAL= 5,
    DATETIME_GREATER_THAN_OR_EQUAL = 6,
    BOOLEAN_EQUAL = 7,
    BOOLEAN_NOT_EQUAL = 8,
}

impl DiscordTypes for ConnectionMetadataType {
    fn from(value: u8) -> Self {
        match value { 
            1 => ConnectionMetadataType::INTEGER_LESS_THAN_OR_EQUAL,
            2 => ConnectionMetadataType::INTEGER_GREATER_THAN_OR_EQUAL,
            3 => ConnectionMetadataType::INTEGER_EQUAL,
            4 => ConnectionMetadataType::INTEGER_NOT_EQUAL,
            5 => ConnectionMetadataType::DATETIME_LESS_THAN_OR_EQUAL,
            6 => ConnectionMetadataType::DATETIME_GREATER_THAN_OR_EQUAL,
            7 => ConnectionMetadataType::BOOLEAN_EQUAL,
            8 => ConnectionMetadataType::BOOLEAN_NOT_EQUAL,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self { 
            ConnectionMetadataType::INTEGER_LESS_THAN_OR_EQUAL => 1,
            ConnectionMetadataType::INTEGER_GREATER_THAN_OR_EQUAL => 2,
            ConnectionMetadataType::INTEGER_EQUAL => 3,
            ConnectionMetadataType::INTEGER_NOT_EQUAL => 4,
            ConnectionMetadataType::DATETIME_LESS_THAN_OR_EQUAL => 5,
            ConnectionMetadataType::DATETIME_GREATER_THAN_OR_EQUAL => 6,
            ConnectionMetadataType::BOOLEAN_EQUAL => 7,
            ConnectionMetadataType::BOOLEAN_NOT_EQUAL => 8,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationRoleConnectionMetadata {
    #[serde(rename = "type")]
    pub format_type: ConnectionMetadataType,
    pub key: String,
    pub name: String,
    pub name_localizations: Option<std::collections::HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<std::collections::HashMap<String, String>>,
}
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
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
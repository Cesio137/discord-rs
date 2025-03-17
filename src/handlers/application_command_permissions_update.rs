use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Permission {
    pub id: String,
    pub type_: u8,
    pub permission: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationCommandPermissionsUpdate {
    pub id: String,
    pub application_id: String,
    pub guild_id: String,
    pub permissions: Vec<Permission>,
}

impl ApplicationCommandPermissionsUpdate {
    pub fn from_value(value: &Value) -> Option<Self> {
        match serde_json::from_value(value.clone()) {
            Ok(permissions_update) => Some(permissions_update),
            Err(err) => {
                eprintln!("Error deserializing ApplicationCommandPermissionsUpdate data: {}", err);
                None
            }
        }
    }
}
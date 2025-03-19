use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum SkuType {
    DURABLE = 2,
    CONSUMABLE = 3,
    SUBSCRIPTION = 5,
    SUBSCRIPTION_GROUP = 6
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sku {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: SkuType,
    pub application_id: String,
    pub name: String,
    pub slug: String,
    pub flags: i32,
}
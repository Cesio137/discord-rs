use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SkuType {
    DURABLE = 2,
    CONSUMABLE = 3,
    SUBSCRIPTION = 5,
    SUBSCRIPTION_GROUP = 6
}

impl DiscordTypes for SkuType {
    fn from(value: u8) -> Self {
        match value {
            2 => SkuType::DURABLE,
            3 => SkuType::CONSUMABLE,
            5 => SkuType::SUBSCRIPTION,
            6 => SkuType::SUBSCRIPTION_GROUP,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            SkuType::DURABLE => 2,
            SkuType::CONSUMABLE => 3,
            SkuType::SUBSCRIPTION => 5,
            SkuType::SUBSCRIPTION_GROUP => 6,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sku {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: SkuType,
    pub application_id: String,
    pub name: String,
    pub slug: String,
    pub flags: i32,
}
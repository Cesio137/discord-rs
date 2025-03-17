use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntitlementType {
    PURCHASE = 1,
    PREMIUM_SUBSCRIPTION = 2,
    DEVELOPER_GIFT = 3,
    TEST_MODE_PURCHASE = 4,
    FREE_PURCHASE = 5,
    USER_GIFT = 6,
    PREMIUM_PURCHASE = 7,
    APPLICATION_SUBSCRIPTION = 8
}

impl DiscordTypes for EntitlementType {
    fn from(value: u8) -> Self {
        match value {
            1 => EntitlementType::PURCHASE,
            2 => EntitlementType::PREMIUM_SUBSCRIPTION,
            3 => EntitlementType::DEVELOPER_GIFT,
            4 => EntitlementType::TEST_MODE_PURCHASE,
            5 => EntitlementType::FREE_PURCHASE,
            6 => EntitlementType::USER_GIFT,
            7 => EntitlementType::PREMIUM_PURCHASE,
            8 => EntitlementType::APPLICATION_SUBSCRIPTION,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            EntitlementType::PURCHASE => 1,
            EntitlementType::PREMIUM_SUBSCRIPTION => 2,
            EntitlementType::DEVELOPER_GIFT => 3,
            EntitlementType::TEST_MODE_PURCHASE => 4,
            EntitlementType::FREE_PURCHASE => 5,
            EntitlementType::USER_GIFT => 6,
            EntitlementType::PREMIUM_PURCHASE => 7,
            EntitlementType::APPLICATION_SUBSCRIPTION => 8,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entitlement {
    pub id: String,
    pub sku_id: String,
    pub application_id: String,
    pub user_id: Option<String>,
    pub promotion_id: Option<String>,
    #[serde(rename = "type")]
    pub format_type: EntitlementType,
    pub deleted: bool,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub guild_id: Option<String>,
    pub consumed: Option<bool>,
}
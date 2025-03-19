use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
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
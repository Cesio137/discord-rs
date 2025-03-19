use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum SubscriptionStatuses {
    ACTIVE = 0,
    ENDING = 1,
    INACTIVE = 2
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscriptionListing {
    pub id: String,
    pub user_id: String,
    pub sku_ids: Vec<String>,
    pub entitlement_ids: Vec<String>,
    pub renewal_sku_ids: Option<Vec<String>>,
    pub current_period_start: String,
    pub current_period_end: String,
    pub status: SubscriptionStatuses,
    pub canceled_at: Option<String>,
    pub country: Option<String>,
}
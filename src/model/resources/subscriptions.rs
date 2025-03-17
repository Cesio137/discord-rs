use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;
/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SubscriptionStatuses {
    ACTIVE = 0,
    ENDING = 1,
    INACTIVE = 2
}

impl DiscordTypes for SubscriptionStatuses {
    fn from(value: u8) -> Self {
        match value { 
            0 => SubscriptionStatuses::ACTIVE,
            1 => SubscriptionStatuses::ENDING,
            2 => SubscriptionStatuses::INACTIVE,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self { 
            SubscriptionStatuses::ACTIVE => 0,
            SubscriptionStatuses::ENDING => 1,
            SubscriptionStatuses::INACTIVE => 2,
        }
    }
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
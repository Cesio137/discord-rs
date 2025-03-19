use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::application;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum IntegrationExpireBehavior {
    RemoveRole = 0,
    Kick = 1,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Integration {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub format_type: String,
    pub enabled: bool,
    pub syncing: Option<bool>,
    pub role_id: Option<String>,
    pub enable_emoticons: Option<bool>,
    pub expire_behavior: Option<IntegrationExpireBehavior>,
    pub expire_grace_period: Option<i32>,
    pub user: Option<super::super::user::User>,
    pub account: IntegrationAccount,
    pub synced_at: Option<String>,
    pub subscriber_count: Option<i32>,
    pub revoked: Option<bool>,
    pub application: Option<application::Application>,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationApplication {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub bot: Option<super::super::user::User>,
}
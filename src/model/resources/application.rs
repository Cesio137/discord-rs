use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::topics::teams::Team;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum IntegrationType {
    GUILD_INSTALL = 0,
    USER_INSTALL = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum EventWebhookStatus {
    DISABLED = 1,
    ENABLED = 2,
    DISABLED_BY_DISCORD = 3,
}

/*FLAGS*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum ApplicationFlags {
    APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 64,
    GATEWAY_PRESENCE = 4096,
    GATEWAY_PRESENCE_LIMITED = 8192,
    GATEWAY_GUILD_MEMBERS = 16384,
    GATEWAY_GUILD_MEMBERS_LIMITED = 32768,
    VERIFICATION_PENDING_GUILD_LIMIT = 65536,
    EMBEDDED = 131072,
    GATEWAY_MESSAGE_CONTENT = 262144,
    GATEWAY_MESSAGE_CONTENT_LIMITED = 524288,
    APPLICATION_COMMAND_BADGE = 8388608,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: String,
    pub rpc_origins: Option<Vec<String>>,
    pub bot_public: bool,
    pub bot_require_code_grant: bool,
    pub bot: Option<super::user::User>,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub owner: Option<super::user::User>,
    pub verify_key: String,
    pub team: Option<Team>,
    pub guild_id: Option<String>,
    pub guild: Option<super::guild::Guild>,
    pub primary_sku_id: Option<String>,
    pub slug: Option<String>,
    pub cover_image: Option<String>,
    pub flags: Option<ApplicationFlags>,
    pub approximate_guild_count: Option<u32>,
    pub approximate_user_install_count: Option<u32>,
    pub redirect_uris: Option<Vec<String>>,
    pub interactions_endpoint_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
    pub event_webhooks_url: Option<String>,
    pub event_webhooks_status: Option<EventWebhookStatus>,
    pub event_webhooks_types: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<InstallParams>,
    pub integration_types_config: Option<HashMap<String, IntegrationType>>,
    pub custom_install_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrationTypeConfiguration {
    pub oauth2_install_params: Option<InstallParams>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::guild::integration::Integration;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PremiumTypes {
    NONE = 0,
    NITRO_CLASSIC = 1,
    NITRO = 2,
    NITRO_BASIC = 3,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VisibilityTypes {
    NONE = 0,
    EVERYONE = 1,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Copy)]
pub enum ServiceType {
    #[serde(rename = "amazon-music")]
    AmazonMusic,
    #[serde(rename = "battlenet")]
    Battlenet,
    #[serde(rename = "bungie")]
    Bungie,
    #[serde(rename = "bluesky")]
    Bluesky,
    #[serde(rename = "crunchyroll")]
    Crunchyroll,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "ebay")]
    Ebay,
    #[serde(rename = "epicgames")]
    EpicGames,
    #[serde(rename = "facebook")]
    Facebook,
    #[serde(rename = "github")]
    Github,
    #[serde(rename = "instagram")]
    Instagram,
    #[serde(rename = "leagueoflegends")]
    Leagueoflegends,
    #[serde(rename = "mastodon")]
    Mastodon,
    #[serde(rename = "paypal")]
    Paypal,
    #[serde(rename = "playstation")]
    Playstation,
    #[serde(rename = "reddit")]
    Reddit,
    #[serde(rename = "riotgames")]
    Riotgames,
    #[serde(rename = "roblox")]
    Roblox,
    #[serde(rename = "spotify")]
    Spotify,
    #[serde(rename = "skype")]
    Skype,
    #[serde(rename = "steam")]
    Steam,
    #[serde(rename = "tiktok")]
    Tiktok,
    #[serde(rename = "twitch")]
    Twitch,
    #[serde(rename = "twitter")]
    Twitter,
    #[serde(rename = "xbox")]
    Xbox,
    #[serde(rename = "youtube")]
    Youtube,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<i32>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<i32>,
    pub premium_type: Option<PremiumTypes>,
    pub public_flags: Option<i32>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AvatarDecorationData {
    pub asset: String,
    pub sku_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub format_type: ServiceType,
    pub revoked: Option<bool>,
    pub integrations: Option<Vec<Integration>>,
    pub verified: bool,
    pub friend_sync: bool,
    pub show_activity: bool,
    pub two_way_link: bool,
    pub visibility: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApplicationRoleConnection {
    pub platform_name: Option<String>,
    pub platform_username: Option<String>,
    pub metadata: super::application_role_connection_metadata_object::ApplicationRoleConnectionMetadata,
}
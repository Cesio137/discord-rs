use serde::{Deserialize, Serialize};
use crate::internal::traits::{DiscordStringTypes, DiscordTypes};
use crate::model::resources::guild::integration::Integration;
/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PremiumTypes {
    NONE = 0,
    NITRO_CLASSIC = 1,
    NITRO = 2,
    NITRO_BASIC = 3,
}

impl DiscordTypes for PremiumTypes {
    fn from(value: u8) -> Self {
        match value { 
            0 => PremiumTypes::NONE,
            1 => PremiumTypes::NITRO_CLASSIC,
            2 => PremiumTypes::NITRO,
            3 => PremiumTypes::NITRO_BASIC,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self { 
            PremiumTypes::NONE => 0,
            PremiumTypes::NITRO_CLASSIC => 1,
            PremiumTypes::NITRO => 2,
            PremiumTypes::NITRO_BASIC => 3,
        }
    }
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VisibilityTypes {
    NONE = 0,
    EVERYONE = 1,
}

impl DiscordTypes for VisibilityTypes {
    fn from(value: u8) -> Self {
        match value { 
            0 => VisibilityTypes::NONE,
            1 => VisibilityTypes::EVERYONE,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self { 
            VisibilityTypes::NONE => 0,
            VisibilityTypes::EVERYONE => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ServiceType {
    AmazonMusic,
    Battlenet,
    Bungie,
    Bluesky,
    Crunchyroll,
    Domain,
    Ebay,
    Epicgames,
    Facebook,
    Github,
    Instagram,
    Leagueoflegends,
    Mastodon,
    Paypal,
    Playstation,
    Reddit,
    Riotgames,
    Roblox,
    Spotify,
    Skype,
    Steam,
    Tiktok,
    Twitch,
    Twitter,
    Xbox,
    Youtube,
}

impl DiscordStringTypes for ServiceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "amazon-music" => ServiceType::AmazonMusic,
            "battlenet" => ServiceType::Battlenet,
            "bungie" => ServiceType::Bungie,
            "bluesky" => ServiceType::Bluesky,
            "crunchyroll" => ServiceType::Crunchyroll,
            "domain" => ServiceType::Domain,
            "ebay" => ServiceType::Ebay,
            "epicgames" => ServiceType::Epicgames,
            "facebook" => ServiceType::Facebook,
            "github" => ServiceType::Github,
            "instagram" => ServiceType::Instagram,
            "leagueoflegends" => ServiceType::Leagueoflegends,
            "mastodon" => ServiceType::Mastodon,
            "paypal" => ServiceType::Paypal,
            "playstation" => ServiceType::Playstation,
            "reddit" => ServiceType::Reddit,
            "riotgames" => ServiceType::Riotgames,
            "roblox" => ServiceType::Roblox,
            "spotify" => ServiceType::Spotify,
            "skype" => ServiceType::Skype,
            "steam" => ServiceType::Steam,
            "tiktok" => ServiceType::Tiktok,
            "twitch" => ServiceType::Twitch,
            "twitter" => ServiceType::Twitter,
            "xbox" => ServiceType::Xbox,
            "youtube" => ServiceType::Youtube,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> String {
        match self {
            ServiceType::AmazonMusic => "amazon-music".to_string(),
            ServiceType::Battlenet => "battlenet".to_string(),
            ServiceType::Bungie => "bungie".to_string(),
            ServiceType::Bluesky => "bluesky".to_string(),
            ServiceType::Crunchyroll => "crunchyroll".to_string(),
            ServiceType::Domain => "domain".to_string(),
            ServiceType::Ebay => "ebay".to_string(),
            ServiceType::Epicgames => "epicgames".to_string(),
            ServiceType::Facebook => "facebook".to_string(),
            ServiceType::Github => "github".to_string(),
            ServiceType::Instagram => "instagram".to_string(),
            ServiceType::Leagueoflegends => "leagueoflegends".to_string(),
            ServiceType::Mastodon => "mastodon".to_string(),
            ServiceType::Paypal => "paypal".to_string(),
            ServiceType::Playstation => "playstation".to_string(),
            ServiceType::Reddit => "reddit".to_string(),
            ServiceType::Riotgames => "riotgames".to_string(),
            ServiceType::Roblox => "roblox".to_string(),
            ServiceType::Spotify => "spotify".to_string(),
            ServiceType::Skype => "skype".to_string(),
            ServiceType::Steam => "steam".to_string(),
            ServiceType::Tiktok => "tiktok".to_string(),
            ServiceType::Twitch => "twitch".to_string(),
            ServiceType::Twitter => "twitter".to_string(),
            ServiceType::Xbox => "xbox".to_string(),
            ServiceType::Youtube => "youtube".to_string(),
        }
    }
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
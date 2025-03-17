pub mod integration;
mod ban;
pub mod scheduled_event;

use serde::{Deserialize, Serialize};
use crate::internal::traits::{DiscordStringTypes, DiscordTypes};

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DefaultMessageNotificationLevel {
    ALL_MESSAGES = 0,
    ONLY_MENTIONS = 1
}

impl DiscordTypes for DefaultMessageNotificationLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => DefaultMessageNotificationLevel::ONLY_MENTIONS,
            1 => DefaultMessageNotificationLevel::ALL_MESSAGES,
            _ => DefaultMessageNotificationLevel::ONLY_MENTIONS,
        }
    }

    fn value(&self) -> u8 {
        match self {
            DefaultMessageNotificationLevel::ONLY_MENTIONS => 0,
            DefaultMessageNotificationLevel::ALL_MESSAGES => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExplicitContentFilterLevel {
    DISABLED = 0,
    MEMBERS_WITHOUT_ROLES = 1,
    ALL_MEMBERS = 2
}

impl DiscordTypes for ExplicitContentFilterLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => ExplicitContentFilterLevel::DISABLED,
            1 => ExplicitContentFilterLevel::MEMBERS_WITHOUT_ROLES,
            2 => ExplicitContentFilterLevel::ALL_MEMBERS,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            ExplicitContentFilterLevel::DISABLED => 0,
            ExplicitContentFilterLevel::MEMBERS_WITHOUT_ROLES => 1,
            ExplicitContentFilterLevel::ALL_MEMBERS => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MFALevel {
    NONE = 0,
    ELEVATED = 1
}

impl DiscordTypes for MFALevel {
    fn from(value: u8) -> Self {
        match value {
            0 => MFALevel::NONE,
            1 => MFALevel::ELEVATED,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            MFALevel::NONE => 0,
            MFALevel::ELEVATED => 1
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VerificationLevel {
    NONE = 0,
    LOW = 1,
    MEDIUM = 2,
    HIGH = 3,
    VERY_HIGH = 4
}

impl DiscordTypes for VerificationLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => VerificationLevel::NONE,
            1 => VerificationLevel::LOW,
            2 => VerificationLevel::MEDIUM,
            3 => VerificationLevel::HIGH,
            4 => VerificationLevel::VERY_HIGH,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            VerificationLevel::NONE => 0,
            VerificationLevel::LOW => 1,
            VerificationLevel::MEDIUM => 2,
            VerificationLevel::HIGH => 3,
            VerificationLevel::VERY_HIGH => 4
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GuildNSFWLevel {
    DEFAULT = 0,
    EXPLICIT = 1,
    SAFE = 2,
    AGE_RESTRICTED = 3
}

impl DiscordTypes for GuildNSFWLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => GuildNSFWLevel::DEFAULT,
            1 => GuildNSFWLevel::EXPLICIT,
            2 => GuildNSFWLevel::SAFE,
            3 => GuildNSFWLevel::AGE_RESTRICTED,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            GuildNSFWLevel::DEFAULT => 0,
            GuildNSFWLevel::EXPLICIT => 1,
            GuildNSFWLevel::SAFE => 2,
            GuildNSFWLevel::AGE_RESTRICTED => 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PremiumTier {
    NONE = 0,
    TIER_1 = 1,
    TIER_2 = 2,
    TIER_3 = 3,
}

impl DiscordTypes for PremiumTier {
    fn from(value: u8) -> Self {
        match value {
            0 => PremiumTier::NONE,
            1 => PremiumTier::TIER_1,
            2 => PremiumTier::TIER_2,
            3 => PremiumTier::TIER_3,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            PremiumTier::NONE => 0,
            PremiumTier::TIER_1 => 1,
            PremiumTier::TIER_2 => 2,
            PremiumTier::TIER_3 => 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GuildFeature {
    AnimatedBanner,
    AnimatedIcon,
    ApplicationCommandPermissionsV2,
    AutoModeration,
    Banner,
    Community,
    CreatorMonetizableProvisional,
    CreatorStorePage,
    DeveloperSupportServer,
    Discoverable,
    Featurable,
    InvitesDisabled,
    InviteSplash,
    MemberVerificationGateEnabled,
    MoreSoundboard,
    MoreStickers,
    News,
    Partnered,
    PreviewEnabled,
    RaidAlertsDisabled,
    RoleIcons,
    RoleSubscriptionsAvailableForPurchase,
    RoleSubscriptionsEnabled,
    Soundboard,
    TicketedEventsEnabled,
    VanityUrl,
    Verified,
    VipRegions,
    WelcomeScreenEnabled,
}

impl DiscordStringTypes for GuildFeature {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ANIMATED_BANNER" => GuildFeature::AnimatedBanner,
            "ANIMATED_ICON" => GuildFeature::AnimatedIcon,
            "APPLICATION_COMMAND_PERMISSIONS_V2" => GuildFeature::ApplicationCommandPermissionsV2,
            "AUTO_MODERATION" => GuildFeature::AutoModeration,
            "BANNER" => GuildFeature::Banner,
            "COMMUNITY" => GuildFeature::Community,
            "CREATOR_MONETIZABLE_PROVISIONAL" => GuildFeature::CreatorMonetizableProvisional,
            "CREATOR_STORE_PAGE" => GuildFeature::CreatorStorePage,
            "DEVELOPER_SUPPORT_SERVER" => GuildFeature::DeveloperSupportServer,
            "DISCOVERABLE" => GuildFeature::Discoverable,
            "FEATURABLE" => GuildFeature::Featurable,
            "INVITES_DISABLED" => GuildFeature::InvitesDisabled,
            "INVITE_SPLASH" => GuildFeature::InviteSplash,
            "MEMBER_VERIFICATION_GATE_ENABLED" => GuildFeature::MemberVerificationGateEnabled,
            "MORE_SOUNDBOARD" => GuildFeature::MoreSoundboard,
            "MORE_STICKERS" => GuildFeature::MoreStickers,
            "NEWS" => GuildFeature::News,
            "PARTNERED" => GuildFeature::Partnered,
            "PREVIEW_ENABLED" => GuildFeature::PreviewEnabled,
            "RAID_ALERTS_DISABLED" => GuildFeature::RaidAlertsDisabled,
            "ROLE_ICONS" => GuildFeature::RoleIcons,
            "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE" => GuildFeature::RoleSubscriptionsAvailableForPurchase,
            "ROLE_SUBSCRIPTIONS_ENABLED" => GuildFeature::RoleSubscriptionsEnabled,
            "SOUNDBOARD" => GuildFeature::Soundboard,
            "TICKETED_EVENTS_ENABLED" => GuildFeature::TicketedEventsEnabled,
            "VANITY_URL" => GuildFeature::VanityUrl,
            "VERIFIED" => GuildFeature::Verified,
            "VIP_REGIONS" => GuildFeature::VipRegions,
            "WELCOME_SCREEN_ENABLED" => GuildFeature::WelcomeScreenEnabled,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> String {
        match self {
            GuildFeature::AnimatedBanner => "ANIMATED_BANNER".to_string(),
            GuildFeature::AnimatedIcon => "ANIMATED_ICON".to_string(),
            GuildFeature::ApplicationCommandPermissionsV2 => "APPLICATION_COMMAND_PERMISSIONS_V2".to_string(),
            GuildFeature::AutoModeration => "AUTO_MODERATION".to_string(),
            GuildFeature::Banner => "BANNER".to_string(),
            GuildFeature::Community => "COMMUNITY".to_string(),
            GuildFeature::CreatorMonetizableProvisional => "CREATOR_MONETIZABLE_PROVISIONAL".to_string(),
            GuildFeature::CreatorStorePage => "CREATOR_STORE_PAGE".to_string(),
            GuildFeature::DeveloperSupportServer => "DEVELOPER_SUPPORT_SERVER".to_string(),
            GuildFeature::Discoverable => "DISCOVERABLE".to_string(),
            GuildFeature::Featurable => "FEATURABLE".to_string(),
            GuildFeature::InvitesDisabled => "INVITES_DISABLED".to_string(),
            GuildFeature::InviteSplash => "INVITE_SPLASH".to_string(),
            GuildFeature::MemberVerificationGateEnabled => "MEMBER_VERIFICATION_GATE_ENABLED".to_string(),
            GuildFeature::MoreSoundboard => "MORE_SOUNDBOARD".to_string(),
            GuildFeature::MoreStickers => "MORE_STICKERS".to_string(),
            GuildFeature::News => "NEWS".to_string(),
            GuildFeature::Partnered => "PARTNERED".to_string(),
            GuildFeature::PreviewEnabled => "PREVIEW_ENABLED".to_string(),
            GuildFeature::RaidAlertsDisabled => "RAID_ALERTS_DISABLED".to_string(),
            GuildFeature::RoleIcons => "ROLE_ICONS".to_string(),
            GuildFeature::RoleSubscriptionsAvailableForPurchase => "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE".to_string(),
            GuildFeature::RoleSubscriptionsEnabled => "ROLE_SUBSCRIPTIONS_ENABLED".to_string(),
            GuildFeature::Soundboard => "SOUNDBOARD".to_string(),
            GuildFeature::TicketedEventsEnabled => "TICKETED_EVENTS_ENABLED".to_string(),
            GuildFeature::VanityUrl => "VANITY_URL".to_string(),
            GuildFeature::Verified => "VERIFIED".to_string(),
            GuildFeature::VipRegions => "VIP_REGIONS".to_string(),
            GuildFeature::WelcomeScreenEnabled => "WELCOME_SCREEN_ENABLED".to_string(),
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub owner_id: String,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: i32,
    pub verification_level: VerificationLevel,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub explicit_content_filter: ExplicitContentFilterLevel,
    pub roles: Vec<Role>,
    pub emojis: Vec<super::emoji::Emoji>,
    pub features: Vec<GuildFeature>,
    pub mfa_level: MFALevel,
    pub application_id: Option<String>,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: i32,
    pub rules_channel_id: Option<String>,
    pub max_presences: Option<i32>,
    pub max_members: Option<i32>,
    pub vanity_url_code: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub premium_tier: PremiumTier,
    pub premium_subscription_count: Option<i32>,
    pub preferred_locale: String,
    pub public_updates_channel_id: Option<String>,
    pub max_video_channel_users: Option<i32>,
    pub approximate_member_count: Option<i32>,
    pub approximate_presence_count: Option<i32>,
    pub welcome_screen: Option<WelcomeScreen>,
    pub nsfw_level: GuildNSFWLevel,
    pub stickers: Option<Vec<super::sticker::Sticker>>,
    pub premium_progress_bar_enabled: bool,
    pub safety_alerts_channel_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildPreview {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    pub emojis: Vec<super::emoji::Emoji>,
    pub features: Vec<String>,
    pub approximate_member_count: i32,
    pub approximate_presence_count: i32,
    pub description: Option<String>,
    pub sticker: Vec<super::sticker::Sticker>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildWidgetSettings {
    pub enabled: bool,
    pub channel_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildWidget {
    pub id: String,
    pub name: String,
    pub instant_invite: Option<String>,
    pub channels: Vec<super::channel::Channel>,
    pub members: Vec<super::user::User>,
    pub presence_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuildMember {
    pub user: Option<super::user::User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: String,
    pub premium_since: Option<String>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: i32,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub avatar_decoration_data: Option<super::user::AvatarDecorationData>,
}

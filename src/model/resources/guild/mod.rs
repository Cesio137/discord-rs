pub mod integration;
pub mod ban;
pub mod scheduled_event;
pub mod welcome_screen;
mod onboarding;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::guild::welcome_screen::WelcomeScreen;
use crate::model::topics::permisssions::Role;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    ALL_MESSAGES = 0,
    ONLY_MENTIONS = 1
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ExplicitContentFilterLevel {
    DISABLED = 0,
    MEMBERS_WITHOUT_ROLES = 1,
    ALL_MEMBERS = 2
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum MFALevel {
    NONE = 0,
    ELEVATED = 1
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VerificationLevel {
    NONE = 0,
    LOW = 1,
    MEDIUM = 2,
    HIGH = 3,
    VERY_HIGH = 4
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GuildNSFWLevel {
    DEFAULT = 0,
    EXPLICIT = 1,
    SAFE = 2,
    AGE_RESTRICTED = 3
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum PremiumTier {
    NONE = 0,
    TIER_1 = 1,
    TIER_2 = 2,
    TIER_3 = 3,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum GuildFeature {
    #[serde(rename = "ANIMATED_BANNER")]
    AnimatedBanner,
    #[serde(rename = "ANIMATED_ICON")]
    AnimatedIcon,
    #[serde(rename = "APPLICATION_COMMAND_PERMISSIONS_V2")]
    ApplicationCommandPermissionsV2,
    #[serde(rename = "AUTO_MODERATION")]
    AutoModeration,
    #[serde(rename = "BANNER")]
    Banner,
    #[serde(rename = "COMMUNITY")]
    Community,
    #[serde(rename = "CREATOR_MONETIZABLE_PROVISIONAL")]
    CreatorMonetizableProvisional,
    #[serde(rename = "CREATOR_STORE_PAGE")]
    CreatorStorePage,
    #[serde(rename = "DEVELOPER_SUPPORT_SERVER")]
    DeveloperSupportServer,
    #[serde(rename = "DISCOVERABLE")]
    Discoverable,
    #[serde(rename = "FEATURABLE")]
    Featurable,
    #[serde(rename = "INVITES_DISABLED")]
    InvitesDisabled,
    #[serde(rename = "INVITE_SPLASH")]
    InviteSplash,
    #[serde(rename = "MEMBER_VERIFICATION_GATE_ENABLED")]
    MemberVerificationGateEnabled,
    #[serde(rename = "MORE_SOUNDBOARD")]
    MoreSoundboard,
    #[serde(rename = "MORE_STICKERS")]
    MoreStickers,
    #[serde(rename = "NEWS")]
    News,
    #[serde(rename = "PARTNERED")]
    Partnered,
    #[serde(rename = "PREVIEW_ENABLED")]
    PreviewEnabled,
    #[serde(rename = "RAID_ALERTS_DISABLED")]
    RaidAlertsDisabled,
    #[serde(rename = "ROLE_ICONS")]
    RoleIcons,
    #[serde(rename = "ROLE_SUBSCRIPTIONS_AVAILABLE_FOR_PURCHASE")]
    RoleSubscriptionsAvailableForPurchase,
    #[serde(rename = "ROLE_SUBSCRIPTIONS_ENABLED")]
    RoleSubscriptionsEnabled,
    #[serde(rename = "SOUNDBOARD")]
    Soundboard,
    #[serde(rename = "TICKETED_EVENTS_ENABLED")]
    TicketedEventsEnabled,
    #[serde(rename = "VANITY_URL")]
    VanityUrl,
    #[serde(rename = "VERIFIED")]
    Verified,
    #[serde(rename = "VIP_REGIONS")]
    VipRegions,
    #[serde(rename = "WELCOME_SCREEN_ENABLED")]
    WelcomeScreenEnabled,
}

/*FLAGS*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum GuildMemberFlag {
    DidRejoin = 1,
    CompletedOnboarding = 2,
    BypassesVerification = 4,
    StartedOnboarding = 8,
    IsGuest = 16,
    StartedHomeActions = 32,
    CompletedHomeActions = 64,
    AutomodQuarantinedUsername = 128,
    DMSettingsUpsellAcknowledged = 512,
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
    pub flags: GuildMemberFlag,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    pub communication_disabled_until: Option<String>,
    pub avatar_decoration_data: Option<super::user::AvatarDecorationData>,
}

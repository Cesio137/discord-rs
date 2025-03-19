use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*FLAGS*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u64)]
pub enum Permission {
    CreateInstantInvite = 1,
    KickMembers = 2,
    BanMembers = 4,
    Administrator = 8,
    ManageChannels = 16,
    ManageGuild = 32,
    AddReactions = 64,
    ViewAuditLog = 128,
    PrioritySpeaker = 256,
    Stream = 512,
    ViewChannel = 1024,
    SendMessages = 2048,
    SendTTSMessages = 4096,
    ManageMessages = 8192,
    EmbedLinks = 16384,
    AttachFiles = 32768,
    ReadMessageHistory = 65536,
    MentionEveryone = 131072,
    UseExternalEmojis = 262144,
    ViewGuildInsights = 524288,
    Connect = 1048576,
    Speak = 2097152,
    MuteMembers = 4194304,
    DeafenMembers = 8388608,
    MoveMembers = 16777216,
    UseVAD = 33554432,
    ChangeNickname = 67108864,
    ManageNicknames = 134217728,
    ManageRoles = 268435456,
    ManageWebhooks = 536870912,
    ManageGuildExpressions = 1073741824,
    UseApplicationCommands = 2147483648,
    RequestToSpeak = 4294967296,
    ManageEvents = 8589934592,
    ManageThreads = 17179869184,
    CreatePublicThreads = 34359738368,
    CreatePrivateThreads = 68719476736,
    UseExternalStickers = 137438953472,
    SendMessagesInThreads = 274877906944,
    UseEmbeddedActivities = 549755813888,
    ModerateMembers = 1099511627776,
    ViewCreatorMonetizationAnalytics = 2199023255552,
    UseSoundboard = 4398046511104,
    CreateGuildExpressions = 8796093022208,
    CreateEvents = 17592186044416,
    UseExternalSounds = 35184372088832,
    SendVoiceMessages = 70368744177664,
    SendPolls = 562949953421312,
    UseExternalApps = 1125899906842624,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub color: i32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: i32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub tags: Option<RoleTags>,
    pub flags: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleTags {
    pub bot_id: Option<String>,
    pub integration_id: Option<String>,
    pub premium_subscriber: Option<()>,
    pub subscription_listing_id: Option<String>,
    pub available_for_purchase: Option<()>,
    pub guild_connections: Option<()>,
}
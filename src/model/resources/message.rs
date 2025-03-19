use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::interacion::message_components::{Component};
use crate::model::interacion::{MessageInteraction};
use crate::model::resources::application::IntegrationType;
use crate::model::resources::user::User;
use crate::model::topics::permisssions::Role;

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum MessageType {
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
    PollResult = 46,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum EmbedType {
    #[serde(rename = "rich")]
    RICH,
    #[serde(rename = "image")]
    IMAGE,
    #[serde(rename = "video")]
    VIDEO,
    #[serde(rename = "gifv")]
    GIFV,
    #[serde(rename = "article")]
    ARTICLE,
    #[serde(rename = "link")]
    LINK,
    #[serde(rename = "poll_result")]
    POLL_RESULT,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum MessageActivityType {
    JOIN = 1,
    SPECTATE = 2,
    LISTEN = 3,
    JOIN_REQUEST = 5,
}

/*FLAGS*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
pub enum MessageFlags {
    CROSSPOSTED = 1,
    IS_CROSSPOST = 2,
    SUPPRESS_EMBEDS = 4,
    SOURCE_MESSAGE_DELETED = 8,
    URGENT = 16,
    HAS_THREAD = 32,
    EPHEMERAL = 64,
    LOADING = 128,
    FAILED_TO_MENTION_SOME_ROLES_IN_THREAD = 256,
    SUPPRESS_NOTIFICATIONS = 4096,
    IS_VOICE_MESSAGE = 8192,
    HAS_SNAPSHOT = 16384,
}

/*TAGS*/
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum MessageInteractionMetadata {
    #[serde(rename = "2")]
    CommandInteraction {
        id: String,
        user: User,
        authorizing_integration_owners: IntegrationType,
        original_response_message_id: Option<String>,
        target_user: Option<User>,
        target_message_id: Option<String>,
    },
    #[serde(rename = "3")]
    ComponentInteraction {
        id: String,
        user: User,
        authorizing_integration_owners: IntegrationType,
        original_response_message_id: Option<String>,
        interacted_message_id: String,
    },
    #[serde(rename = "5")]
    ModalSubmit {
        id: String,
        user: User,
        authorizing_integration_owners: IntegrationType,
        original_response_message_id: Option<String>,
        triggering_interaction_metadata: MessageInteractionMetadata,
    },
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<User>,
    pub mention_roles: Vec<String>,
    pub mention_channels: Option<Vec<ChannelMention>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub format_type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<super::application::Application>,
    pub application_id: Option<String>,
    pub flags: Option<MessageFlags>,
    pub message_reference: Option<Message>,
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    pub referenced_message: Option<Box<Message>>,
    pub interaction_metadata: Option<MessageInteractionMetadata>,
    pub interaction: Option<MessageInteraction>,
    pub thread: Option<super::channel::Channel>,
    pub components: Option<Vec<Component>>,
    pub sticker_items: Option<Vec<super::sticker::StickerItem>>,
    pub stickers: Option<Vec<super::sticker::Sticker>>,
    pub position: Option<i32>,
    pub role_subscription_data: Option<RoleSubscriptionData>,
    pub resolved: Option<ResolvedData>,
    pub poll: Option<super::poll::Poll>,
    pub call: Option<MessageCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageActivity {
    pub format_type: MessageActivityType,
    pub party_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageSnapshot {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolvedData {
    pub users: Option<Vec<User>>,
    pub members: Option<Vec<super::guild::GuildMember>>,
    pub channels: Option<Vec<super::channel::Channel>>,
    pub roles: Option<Vec<Role>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageCall {
    pub ended_timestamp: Option<String>,
    pub participants: Option<Vec<super::user::User>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelMention {
    pub id: String,
    pub guild_id: String,
    pub format_type: super::channel::ChannelType,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: i32,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<i32>,
    pub width: Option<i32>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f32>,
    pub waveform: Option<String>,
    pub flags: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reaction {
    pub count: i32,
    pub count_details: Option<ReactionCountDetails>,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: super::emoji::Emoji,
    pub burst_colors: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ReactionCountDetails {
    pub burst: i32,
    pub normal: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub format_type: Option<EmbedType>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<String>,
    pub color: Option<i32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedThumbnail {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedVideo {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedImage {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<i32>,
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedAuthor {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleSubscriptionData {
    pub role_subscription_listing_id: String,
    pub tier_name: String,
    pub total_months_subscribed: i32,
    pub is_renewal: bool,
}
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::internal::traits::{DiscordFlagsTypes, DiscordStringTypes, DiscordTypes};
use crate::model::interacion::message_components::Component;
use crate::model::interacion::{InteractionType, MessageInteraction};
/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

impl DiscordTypes for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::Default,
            1 => MessageType::RecipientAdd,
            2 => MessageType::RecipientRemove,
            3 => MessageType::Call,
            4 => MessageType::ChannelNameChange,
            5 => MessageType::ChannelIconChange,
            6 => MessageType::ChannelPinnedMessage,
            7 => MessageType::UserJoin,
            8 => MessageType::GuildBoost,
            9 => MessageType::GuildBoostTier1,
            10 => MessageType::GuildBoostTier2,
            11 => MessageType::GuildBoostTier3,
            12 => MessageType::ChannelFollowAdd,
            14 => MessageType::GuildDiscoveryDisqualified,
            15 => MessageType::GuildDiscoveryRequalified,
            16 => MessageType::GuildDiscoveryGracePeriodInitialWarning,
            17 => MessageType::GuildDiscoveryGracePeriodFinalWarning,
            18 => MessageType::ThreadCreated,
            19 => MessageType::Reply,
            20 => MessageType::ChatInputCommand,
            21 => MessageType::ThreadStarterMessage,
            22 => MessageType::GuildInviteReminder,
            23 => MessageType::ContextMenuCommand,
            24 => MessageType::AutoModerationAction,
            25 => MessageType::RoleSubscriptionPurchase,
            26 => MessageType::InteractionPremiumUpsell,
            27 => MessageType::StageStart,
            28 => MessageType::StageEnd,
            29 => MessageType::StageSpeaker,
            31 => MessageType::StageTopic,
            32 => MessageType::GuildApplicationPremiumSubscription,
            36 => MessageType::GuildIncidentAlertModeEnabled,
            37 => MessageType::GuildIncidentAlertModeDisabled,
            38 => MessageType::GuildIncidentReportRaid,
            39 => MessageType::GuildIncidentReportFalseAlarm,
            44 => MessageType::PurchaseNotification,
            46 => MessageType::PollResult,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            MessageType::Default => 0,
            MessageType::RecipientAdd => 1,
            MessageType::RecipientRemove => 2,
            MessageType::Call => 3,
            MessageType::ChannelNameChange => 4,
            MessageType::ChannelIconChange => 5,
            MessageType::ChannelPinnedMessage => 6,
            MessageType::UserJoin => 7,
            MessageType::GuildBoost => 8,
            MessageType::GuildBoostTier1 => 9,
            MessageType::GuildBoostTier2 => 10,
            MessageType::GuildBoostTier3 => 11,
            MessageType::ChannelFollowAdd => 12,
            MessageType::GuildDiscoveryDisqualified => 14,
            MessageType::GuildDiscoveryRequalified => 15,
            MessageType::GuildDiscoveryGracePeriodInitialWarning => 16,
            MessageType::GuildDiscoveryGracePeriodFinalWarning => 17,
            MessageType::ThreadCreated => 18,
            MessageType::Reply => 19,
            MessageType::ChatInputCommand => 20,
            MessageType::ThreadStarterMessage => 21,
            MessageType::GuildInviteReminder => 22,
            MessageType::ContextMenuCommand => 23,
            MessageType::AutoModerationAction => 24,
            MessageType::RoleSubscriptionPurchase => 25,
            MessageType::InteractionPremiumUpsell => 26,
            MessageType::StageStart => 27,
            MessageType::StageEnd => 28,
            MessageType::StageSpeaker => 29,
            MessageType::StageTopic => 31,
            MessageType::GuildApplicationPremiumSubscription => 32,
            MessageType::GuildIncidentAlertModeEnabled => 36,
            MessageType::GuildIncidentAlertModeDisabled => 37,
            MessageType::GuildIncidentReportRaid => 38,
            MessageType::GuildIncidentReportFalseAlarm => 39,
            MessageType::PurchaseNotification => 44,
            MessageType::PollResult => 46,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EmbedType {
    RICH,
    IMAGE,
    VIDEO,
    GIFV,
    ARTICLE,
    LINK,
    POLL_RESULT,
}

impl DiscordStringTypes for EmbedType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "rich" => EmbedType::RICH,
            "image" => EmbedType::IMAGE,
            "video" => EmbedType::VIDEO,
            "gifv" => EmbedType::GIFV,
            "article" => EmbedType::ARTICLE,
            "link" => EmbedType::LINK,
            "poll_result" => EmbedType::POLL_RESULT,
            _ => unreachable!()
        }
    }

    fn value(&self) -> String {
        match self {
            EmbedType::RICH => String::from("rich"),
            EmbedType::IMAGE => String::from("image"),
            EmbedType::VIDEO => String::from("video"),
            EmbedType::GIFV => String::from("gifv"),
            EmbedType::ARTICLE => String::from("article"),
            EmbedType::LINK => String::from("link"),
            EmbedType::POLL_RESULT => String::from("poll_result"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MessageActivityType {
    JOIN = 1,
    SPECTATE = 2,
    LISTEN = 3,
    JOIN_REQUEST = 5,
}

impl DiscordTypes for MessageActivityType {
    fn from(value: u8) -> Self {
        match value {
            1 => MessageActivityType::JOIN,
            2 => MessageActivityType::SPECTATE,
            3 => MessageActivityType::LISTEN,
            5 => MessageActivityType::JOIN_REQUEST,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            MessageActivityType::JOIN => 1,
            MessageActivityType::SPECTATE => 2,
            MessageActivityType::LISTEN => 3,
            MessageActivityType::JOIN_REQUEST => 5
        }
    }
}

/*FLAGS*/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

impl DiscordFlagsTypes for MessageFlags {
    fn from(value: i32) -> Self {
        match value { 
            1 => MessageFlags::CROSSPOSTED,
            2 => MessageFlags::IS_CROSSPOST,
            4 => MessageFlags::SUPPRESS_EMBEDS,
            8 => MessageFlags::SOURCE_MESSAGE_DELETED,
            16 => MessageFlags::URGENT,
            32 => MessageFlags::HAS_THREAD,
            64 => MessageFlags::EPHEMERAL,
            128 => MessageFlags::LOADING,
            256 => MessageFlags::FAILED_TO_MENTION_SOME_ROLES_IN_THREAD,
            4096 => MessageFlags::SUPPRESS_NOTIFICATIONS,
            8192 => MessageFlags::IS_VOICE_MESSAGE,
            16384 => MessageFlags::HAS_SNAPSHOT,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> i32 {
        match self { 
            MessageFlags::CROSSPOSTED => 1,
            MessageFlags::IS_CROSSPOST => 2,
            MessageFlags::SUPPRESS_EMBEDS => 4,
            MessageFlags::SOURCE_MESSAGE_DELETED => 8,
            MessageFlags::URGENT => 16,
            MessageFlags::HAS_THREAD => 32,
            MessageFlags::EPHEMERAL => 64,
            MessageFlags::LOADING => 128,
            MessageFlags::FAILED_TO_MENTION_SOME_ROLES_IN_THREAD => 256,
            MessageFlags::SUPPRESS_NOTIFICATIONS => 4096,
            MessageFlags::IS_VOICE_MESSAGE => 8192,
            MessageFlags::HAS_SNAPSHOT => 16384,
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub channel_id: String,
    pub author: super::user::User,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Vec<super::user::User>,
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
pub struct MessageInteractionMetadata {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: InteractionType,
    pub user: super::user::User,
    pub authorizing_integration_owners: u8,//
    pub original_response_message_id: Option<String>,
    pub target_user: super::user::User,
    // Message Component Interaction Metadata
    pub interacted_message_id: Option<String>,
    pub triggering_interaction_metadata: Option<MessageInteractionMetadata>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageActivity {
    pub format_type: MessageActivityType,
    pub party_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageSnapshot {
    pub id: String,
    pub channel_id: String,
    pub content: String,
    pub timestamp: String,
    pub edited_timestamp: Option<String>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mention_roles: Vec<String>,
    pub attachments: Vec<super::attachment::Attachment>,
    pub embeds: Vec<super::embed::Embed>,
    pub nonce: Option<String>,
    pub pinned: bool,
    pub webhook_id: Option<String>,
    #[serde(rename = "type")]
    pub type_: u8,
    pub application_id: Option<String>,
    pub flags: Option<i32>,
    pub interaction: Option<super::message_interaction::MessageInteraction>,
    pub components: Option<Vec<super::component::Component>>,
    pub sticker_items: Option<Vec<super::sticker_item::StickerItem>>,
    pub position: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResolvedData {
    pub users: Option<Vec<super::user::User>>,
    pub members: Option<Vec<super::guild_member::GuildMember>>,
    pub channels: Option<Vec<super::channel::Channel>>,
    pub roles: Option<Vec<super::role::Role>>,
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
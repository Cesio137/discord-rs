use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::interacion::application_commands::ApplicationCommand;

/*EVENTS*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum AuditLogEvents {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
    ApplicationCommandPermissionUpdate = 121,
    SoundboardSoundCreate = 130,
    SoundboardSoundUpdate = 131,
    SoundboardSoundDelete = 132,
    AutoModerationRuleCreate = 140,
    AutoModerationRuleUpdate = 141,
    AutoModerationRuleDelete = 142,
    AutoModerationBlockMessage = 143,
    AutoModerationFlagToChannel = 144,
    AutoModerationUserCommunicationDisabled = 145,
    CreatorMonetizationRequestCreated = 150,
    CreatorMonetizationTermsAccepted = 151,
    OnboardingPromptCreate = 163,
    OnboardingPromptUpdate = 164,
    OnboardingPromptDelete = 165,
    OnboardingCreate = 166,
    OnboardingUpdate = 167,
    HomeSettingsCreate = 190,
    HomeSettingsUpdate = 191,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLog {
    pub application_commands: Vec<ApplicationCommand>,
    pub audit_log_entries: Vec<AuditLogChange>,
    pub auto_moderation_rules: Vec<super::auto_moderation::AutoModerationRule>,
    pub guild_scheduled_events: Vec<super::guild::scheduled_event::GuildScheduledEvent>,
    pub integrations: Vec<super::guild::integration::Integration>,
    pub threads: Vec<super::channel::Channel>,
    pub users: Vec<super::user::User>,
    pub webhooks: Vec<super::webhook::Webhook>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogChange {
    pub new_value: Option<serde_json::Value>,
    pub old_value: Option<serde_json::Value>,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogEntry {
    pub target_id: Option<String>,
    pub changes: Option<Vec<AuditLogChange>>,
    pub user_id: Option<String>,
    pub id: String,
    pub action_type: AuditLogEvents,
    pub options: Option<OptionalAuditEntryInfo>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OptionalAuditEntryInfo {
    pub application_id: Option<String>,
    pub auto_moderation_rule_name: Option<String>,
    pub auto_moderation_rule_trigger_type: Option<String>,
    pub channel_id: Option<String>,
    pub count: Option<String>,
    pub delete_member_days: Option<String>,
    pub id: Option<String>,
    pub members_removed: Option<String>,
    pub message_id: Option<String>,
    pub role_name: Option<String>,
    pub format_type: Option<String>,
    pub integration_type: Option<String>,
}
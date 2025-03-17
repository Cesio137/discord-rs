use serde_json::Value;
use crate::handlers::{
    ready::Ready, 
    application_command_permissions_update::ApplicationCommandPermissionsUpdate, 
    auto_moderation_rule::AutoModerationRule
};

pub enum EGatewayEvent {
    Dispatch(EClientEvent),
    ReconnectRequired,
    InvalidSession(bool),
    Close(tokio_tungstenite::tungstenite::protocol::CloseFrame)
}

#[derive(Debug, Clone)]
pub enum EClientEvent {
    None,
    Ready(Ready),
    Resumed,
    ApplicationCommandPermissionsUpdate(ApplicationCommandPermissionsUpdate),
    AutoModerationRuleCreate(AutoModerationRule),
    AutoModerationRuleUpdate(AutoModerationRule),
    AutoModerationRuleDelete(AutoModerationRule),
    AutoModerationRuleExecution(AutoModerationRule),
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    PresenceUpdate,
    TypingStart,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InviteCreate,
    InviteDelete,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
    InteractionCreate,
}

impl EClientEvent {
    fn from(event_type: &str, d: &Value) -> Self {
        match event_type {
            "READY" => {
                let data = match Ready::from_value(d) {
                    Some(data) => data,
                    None => return EClientEvent::None
                };
                EClientEvent::Ready(data)
            },
            "RESUMED" => EClientEvent::Resumed,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => {
                let data = match ApplicationCommandPermissionsUpdate::from_value(d) {
                    Some(data) => data,
                    None => return EClientEvent::None
                };
                EClientEvent::ApplicationCommandPermissionsUpdate(data)
            },
            "AUTO_MODERATION_RULE_CREATE" => {
                let data = match AutoModerationRule::from_value(d) {
                    Some(data) => data,
                    None => return EClientEvent::None
                };
                EClientEvent::AutoModerationRuleCreate(data)
            },
            "AUTO_MODERATION_RULE_UPDATE" => {
                let data = match AutoModerationRule::from_value(d) {
                    Some(data) => data,
                    None => return EClientEvent::None
                };
                EClientEvent::AutoModerationRuleUpdate(data)
            },
            "AUTO_MODERATION_RULE_DELETE" => {
                let data = match AutoModerationRule::from_value(d) {
                    Some(data) => data,
                    None => return EClientEvent::None
                };
                EClientEvent::AutoModerationRuleDelete(data)
            },
            "AUTO_MODERATION_RULE_EXECUTION" => {
                let data = match AutoModerationRule::from_value(d) {
                    Some(data) => data,
                    None => return EClientEvent::None
                };
                EClientEvent::AutoModerationRuleExecution(data)
            },
            "GUILD_CREATE" => EClientEvent::GuildCreate,
            "GUILD_UPDATE" => EClientEvent::GuildUpdate,
            "GUILD_DELETE" => EClientEvent::GuildDelete,
            "GUILD_BAN_ADD" => EClientEvent::GuildBanAdd,
            "GUILD_BAN_REMOVE" => EClientEvent::GuildBanRemove,
            "GUILD_EMOJIS_UPDATE" => EClientEvent::GuildEmojisUpdate,
            "GUILD_INTEGRATIONS_UPDATE" => EClientEvent::GuildIntegrationsUpdate,
            "GUILD_MEMBER_ADD" => EClientEvent::GuildMemberAdd,
            "GUILD_MEMBER_REMOVE" => EClientEvent::GuildMemberRemove,
            "GUILD_MEMBER_UPDATE" => EClientEvent::GuildMemberUpdate,
            "GUILD_MEMBERS_CHUNK" => EClientEvent::GuildMembersChunk,
            "GUILD_ROLE_CREATE" => EClientEvent::GuildRoleCreate,
            "GUILD_ROLE_UPDATE" => EClientEvent::GuildRoleUpdate,
            "GUILD_ROLE_DELETE" => EClientEvent::GuildRoleDelete,
            "CHANNEL_CREATE" => EClientEvent::ChannelCreate,
            "CHANNEL_UPDATE" => EClientEvent::ChannelUpdate,
            "CHANNEL_DELETE" => EClientEvent::ChannelDelete,
            "CHANNEL_PINS_UPDATE" => EClientEvent::ChannelPinsUpdate,
            "THREAD_CREATE" => EClientEvent::ThreadCreate,
            "THREAD_UPDATE" => EClientEvent::ThreadUpdate,
            "THREAD_DELETE" => EClientEvent::ThreadDelete,
            "THREAD_LIST_SYNC" => EClientEvent::ThreadListSync,
            "THREAD_MEMBER_UPDATE" => EClientEvent::ThreadMemberUpdate,
            "THREAD_MEMBERS_UPDATE" => EClientEvent::ThreadMembersUpdate,
            "STAGE_INSTANCE_CREATE" => EClientEvent::StageInstanceCreate,
            "STAGE_INSTANCE_UPDATE" => EClientEvent::StageInstanceUpdate,
            "STAGE_INSTANCE_DELETE" => EClientEvent::StageInstanceDelete,
            "PRESENCE_UPDATE" => EClientEvent::PresenceUpdate,
            "TYPING_START" => EClientEvent::TypingStart,
            "MESSAGE_CREATE" => EClientEvent::MessageCreate,
            "MESSAGE_UPDATE" => EClientEvent::MessageUpdate,
            "MESSAGE_DELETE" => EClientEvent::MessageDelete,
            "MESSAGE_DELETE_BULK" => EClientEvent::MessageDeleteBulk,
            "MESSAGE_REACTION_ADD" => EClientEvent::MessageReactionAdd,
            "MESSAGE_REACTION_REMOVE" => EClientEvent::MessageReactionRemove,
            "MESSAGE_REACTION_REMOVE_ALL" => EClientEvent::MessageReactionRemoveAll,
            "MESSAGE_REACTION_REMOVE_EMOJI" => EClientEvent::MessageReactionRemoveEmoji,
            "INTEGRATION_CREATE" => EClientEvent::IntegrationCreate,
            "INTEGRATION_UPDATE" => EClientEvent::IntegrationUpdate,
            "INTEGRATION_DELETE" => EClientEvent::IntegrationDelete,
            "INVITE_CREATE" => EClientEvent::InviteCreate,
            "INVITE_DELETE" => EClientEvent::InviteDelete,
            "VOICE_STATE_UPDATE" => EClientEvent::VoiceStateUpdate,
            "VOICE_SERVER_UPDATE" => EClientEvent::VoiceServerUpdate,
            "WEBHOOKS_UPDATE" => EClientEvent::WebhooksUpdate,
            "INTERACTION_CREATE" => EClientEvent::InteractionCreate,
            _ => EClientEvent::None,
        }
    }
}
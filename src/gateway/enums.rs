use tungstenite::{protocol::CloseFrame, Utf8Bytes};

pub enum EWebsocketMessage {
    None,
    Text(Utf8Bytes),
    Close(Option<CloseFrame>)
}

pub enum EGatewayEvent {
    Dispatch(EClientEvent),
    ReconnectRequired,
    InvalidSession(bool),
    Close(Option<CloseFrame>)
}

#[repr(u64)]
#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    StatusUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11,
}

impl TryFrom<u64> for Opcode {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Dispatch),
            1 => Ok(Opcode::Heartbeat),
            2 => Ok(Opcode::Identify),
            3 => Ok(Opcode::StatusUpdate),
            4 => Ok(Opcode::VoiceStateUpdate),
            6 => Ok(Opcode::Resume),
            7 => Ok(Opcode::Reconnect),
            8 => Ok(Opcode::RequestGuildMembers),
            9 => Ok(Opcode::InvalidSession),
            10 => Ok(Opcode::Hello),
            11 => Ok(Opcode::HeartbeatAck),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum EClientEvent {
    None,
    Ready,
    Resumed,
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
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
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
    Other(String),
}

impl From<&str> for EClientEvent {
    fn from(event_type: &str) -> Self {
        match event_type {
            "READY" => EClientEvent::Ready,
            "RESUMED" => EClientEvent::Resumed,
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
            other => EClientEvent::Other(other.to_string()),
        }
    }
}


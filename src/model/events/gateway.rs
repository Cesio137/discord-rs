use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use crate::model::resources::application::Application;
use crate::model::resources::guild::Guild;
use crate::model::resources::user::User;
/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
#[repr(u8)]
pub enum GatewayOpcode {
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

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Watching = 3,
    Custom = 4,
    Competing = 5,
}

/*TAGS*/


/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payload {
    pub op: GatewayOpcode,
    pub d: Option<serde_json::Value>,
    pub s: Option<u8>,
    pub t: Option<u32>
}

impl Default for Payload {
    fn default() -> Self {
        Self {
            op: GatewayOpcode::Hello,
            d: None,
            s: None,
            t: None
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Identify {
    pub token: String,
    pub properties: IdentifyConnectionProperties,
    pub compress: Option<bool>,
    pub large_threshold: Option<u16>,
    pub shard: Option<[u16; 2]>,
    pub presence: Option<GatewayPresenceUpdate>,
    pub intents: u32,
}

impl Default for Identify {
    fn default() -> Self {
        Self {
            token: String::from(""),
            properties: {
                IdentifyConnectionProperties::default()
            },
            compress: None,
            large_threshold: None,
            shard: None,
            presence: None,
            intents: 513
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdentifyConnectionProperties {
    pub os: String,
    pub browser: String,
    pub device: String,
}

impl Default for IdentifyConnectionProperties {
    fn default() -> Self {
        Self {
            os: sys_info::os_type().unwrap_or(String::from("unknown")),
            browser: String::from("disco"),
            device: String::from("disco")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GatewayPresenceUpdate {
    pub since: Option<u64>,
    pub activities: Vec<Activity>, // Assuming Activity is defined elsewhere
    pub status: String,
    pub afk: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub format_type: ActivityType,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamps>,
    pub application_id: Option<String>, // Snowflake is represented as String
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<ActivityEmoji>, // Assuming you have an Emoji struct
    pub party: Option<ActivityParty>,
    pub assets: Option<ActivityAssets>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u32>, // Integer flags
    pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityTimestamps {
    pub start: Option<u32>,
    pub end: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityEmoji {
    name: String,
    id: Option<String>,
    animated: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityParty {
    id: Option<String>,
    size: Option<[usize; 2]>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    pub match_: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}

/* SEND EVENTS */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReceivedEvent {
    None,
    Ready(Ready),
    Resumed,
    ApplicationCommandPermissionsUpdate,
    AutoModerationRuleCreate,
    AutoModerationRuleUpdate,
    AutoModerationRuleDelete,
    AutoModerationRuleExecution,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ready {
    pub v: u8,
    pub user: User,
    pub guilds: Vec<Guild>,
    pub session_id: String,
    pub resume_gateway_url: String,
    pub shard: Option<[u16; 2]>,
    pub application: Application,
}
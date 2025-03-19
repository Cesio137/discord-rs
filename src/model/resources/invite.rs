use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum InviteType {
    Guild = 0,
    GroupDm = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum InviteTargetType {
    Stream = 1,
    EmbeddedApplication = 2,
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invite {
    #[serde(rename = "type")]
    pub format_type: Option<InviteType>,
    pub code: String,
    pub guild: Option<super::guild::Guild>,
    pub channel: Option<super::channel::Channel>,
    pub inviter: Option<super::user::User>,
    pub target_type: Option<InviteTargetType>,
    pub target_user: Option<super::user::User>,
    pub target_application: Option<super::application::Application>,
    pub approximate_presence_count: Option<i32>,
    pub approximate_member_count: Option<i32>,
    pub expires_at: Option<String>,
    pub stage_instance: Option<InviteStageInstance>,
    pub guild_scheduled_event: Option<super::guild::scheduled_event::GuildScheduledEvent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InviteStageInstance {
    pub members: Vec<super::user::User>,
    pub participant_count: i32,
    pub speaker_count: i32,
    pub topic: String,
}

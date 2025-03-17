use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InviteType {
    Guild = 0,
    GroupDm = 1,
}

impl DiscordTypes for InviteType {
    fn from(value: u8) -> Self {
        match value {
            0 => InviteType::Guild,
            1 => InviteType::GroupDm,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            InviteType::Guild => 0,
            InviteType::GroupDm => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InviteTargetType {
    Stream = 1,
    EmbeddedApplication = 2,
}

impl DiscordTypes for InviteTargetType {
    fn from(value: u8) -> Self {
        match value {
            1 => InviteTargetType::Stream,
            2 => InviteTargetType::EmbeddedApplication,
            _ => unreachable!(),
        }
    }

    fn value(&self) -> u8 {
        match self {
            InviteTargetType::Stream => 1,
            InviteTargetType::EmbeddedApplication => 2,
        }
    }
}

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

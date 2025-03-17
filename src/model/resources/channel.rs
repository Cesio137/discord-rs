use serde::{Deserialize, Serialize};
use crate::internal::traits::DiscordTypes;

/*TYPES*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelType {
    GUILD_TEXT = 0,
    DM = 1,
    GUILD_VOICE = 2,
    GROUP_DM = 3,
    GUILD_CATEGORY = 4,
    GUILD_ANNOUNCEMENT = 5,
    ANNOUNCEMENT_THREAD = 10,
    PUBLIC_THREAD = 11,
    PRIVATE_THREAD = 12,
    GUILD_STAGE_VOICE = 13,
    GUILD_DIRECTORY = 14,
    GUILD_FORUM = 15,
    GUILD_MEDIA = 16
}

impl DiscordTypes for ChannelType {
    fn from(value: u8) -> Self {
        match value {
            0 => ChannelType::GUILD_TEXT,
            1 => ChannelType::DM,
            2 => ChannelType::GUILD_VOICE,
            3 => ChannelType::GROUP_DM,
            4 => ChannelType::GUILD_CATEGORY,
            5 => ChannelType::GUILD_ANNOUNCEMENT,
            10 => ChannelType::ANNOUNCEMENT_THREAD,
            11 => ChannelType::PUBLIC_THREAD,
            12 => ChannelType::PRIVATE_THREAD,
            13 => ChannelType::GUILD_STAGE_VOICE,
            14 => ChannelType::GUILD_DIRECTORY,
            15 => ChannelType::GUILD_FORUM,
            16 => ChannelType::GUILD_MEDIA,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            ChannelType::GUILD_TEXT => 0,
            ChannelType::DM => 1,
            ChannelType::GUILD_VOICE => 2,
            ChannelType::GROUP_DM => 3,
            ChannelType::GUILD_CATEGORY => 4,
            ChannelType::GUILD_ANNOUNCEMENT => 5,
            ChannelType::ANNOUNCEMENT_THREAD => 10,
            ChannelType::PUBLIC_THREAD => 11,
            ChannelType::PRIVATE_THREAD => 12,
            ChannelType::GUILD_STAGE_VOICE => 13,
            ChannelType::GUILD_DIRECTORY => 14,
            ChannelType::GUILD_FORUM => 15,
            ChannelType::GUILD_MEDIA => 16
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VideoQualityMode {
    AUTO = 1,
    FULL = 2,
}

impl DiscordTypes for VideoQualityMode {
    fn from(value: u8) -> Self {
        match value {
            0 => VideoQualityMode::AUTO,
            1 => VideoQualityMode::FULL,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            VideoQualityMode::AUTO => 0,
            VideoQualityMode::FULL => 1
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SortOrderType {
    LATEST_ACTIVITY = 0,
    CREATION_DATEL = 1,
}

impl DiscordTypes for SortOrderType {
    fn from(value: u8) -> Self {
        match value {
            0 => SortOrderType::LATEST_ACTIVITY,
            1 => SortOrderType::CREATION_DATEL,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            SortOrderType::LATEST_ACTIVITY => 0,
            SortOrderType::CREATION_DATEL => 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ForumLayoutType {
    NOT_SET= 0,
    LIST_VIEW = 1,
    GALLERY_VIEW = 2
}

impl DiscordTypes for ForumLayoutType {
    fn from(value: u8) -> Self {
        match value {
            0 => ForumLayoutType::NOT_SET,
            1 => ForumLayoutType::LIST_VIEW,
            2 => ForumLayoutType::GALLERY_VIEW,
            _ => unreachable!()
        }
    }
    
    fn value(&self) -> u8 {
        match self {
            ForumLayoutType::NOT_SET => 0,
            ForumLayoutType::LIST_VIEW => 1,
            ForumLayoutType::GALLERY_VIEW => 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OverwriteType {
    ROLE = 0,
    MEMBER = 1
}

impl DiscordTypes for OverwriteType {
    fn from(value: u8) -> Self {
        match value {
            0 => OverwriteType::ROLE,
            1 => OverwriteType::MEMBER,
            _ => unreachable!()
        }
    }

    fn value(&self) -> u8 {
        match self {
            OverwriteType::ROLE => 0,
            OverwriteType::MEMBER => 1
        }
    }
}

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Channel {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: ChannelType,
    pub guild_id: Option<String>,
    pub position: Option<i32>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    pub last_message_id: Option<String>,
    pub bitrate: Option<i32>,
    pub user_limit: Option<i32>,
    pub rate_limit_per_user: Option<i32>,
    pub recipients: Option<Vec<super::user::User>>,
    pub icon: Option<String>,
    pub owner_id: Option<String>,
    pub application_id: Option<String>,
    pub managed: Option<bool>,
    pub parent_id: Option<String>,
    pub last_pin_timestamp: Option<String>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<VideoQualityMode>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<i32>,
    pub permissions: Option<String>,
    pub flags: Option<i32>,
    pub total_message_sent: Option<i32>,
    pub available_tags: Option<Vec<ForumTag>>,
    pub applied_tags: Option<Vec<String>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<i32>,
    pub default_sort_order: Option<SortOrderType>,
    pub default_forum_layout: Option<ForumLayoutType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: i32,
    pub archive_timestamp: String,
    pub locked: bool,
    pub invitable: Option<bool>,
    pub create_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadMember {
    pub id: Option<String>,
    pub user_id: Option<String>,
    pub join_timestamp: String,
    pub flags: i32,
    pub member: Option<super::guild::GuildMember>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForumTag {
    pub id: String,
    pub name: String,
    pub moderated: bool,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefaultReaction {
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Overwrite {
    pub id: String,
    #[serde(rename = "type")]
    pub format_type: OverwriteType,
    pub allow: String,
    pub deny: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoundboardSound {
    pub name: String,
    pub sound_id: String,
    pub volume: f32,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
    pub guild_id: Option<String>,
    pub avaliable: bool,
    pub user: Option<super::user::User>,
}
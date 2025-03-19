use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WelcomeScreen {
    pub description: Option<String>,
    pub welcome_channels: Vec<WelcomeScreenChannel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WelcomeScreenChannel {
    pub channel_id: String,
    pub description: String,
    pub emoji_id: Option<String>,
    pub emoji_name: Option<String>,
}
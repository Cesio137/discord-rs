use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*STRUCT OBJECT*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lobby {
    pub id: String,
    pub application_id: String,
    pub metadata: Option<HashMap<String, String>>,
    pub members: Vec<LobbyMember>,
    pub linked_channel: Option<super::channel::Channel>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LobbyMember {
    pub id: String,
    pub metadata: Option<HashMap<String, String>>,
    pub flags: Option<i32>,
}
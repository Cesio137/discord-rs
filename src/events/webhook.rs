use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventBody {
    #[serde(rename = "type")]
    pub type_: u8,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeWebhook {
    PING = 0,
    Event = 1
}

impl TypeWebhook {
    pub fn value(&self) -> u8 {
        match self { 
            TypeWebhook::PING => 0,
            TypeWebhook::Event => 1
        }
    }
}

impl From<u8> for TypeWebhook {
    fn from(value: u8) -> Self {
        match value { 
            0 => TypeWebhook::PING,
            1 => TypeWebhook::Event,
            _ => TypeWebhook::PING
        }
    }
}
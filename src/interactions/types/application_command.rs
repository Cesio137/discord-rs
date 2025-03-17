use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TypeAplicationCommand {
    CHAT_INPUT = 1,
    USER = 2,
    MESSAGE = 3,
    PRIMARY_ENTRY_POINT = 4,
}

impl TypeAplicationCommand {
    pub fn value(&self) -> u8 {
        match self {
            TypeAplicationCommand::CHAT_INPUT => 1,
            TypeAplicationCommand::USER => 2,
            TypeAplicationCommand::MESSAGE => 3,
            TypeAplicationCommand::PRIMARY_ENTRY_POINT => 4,
        }
    }
}

impl From<u8> for TypeAplicationCommand {
    fn from(value: u8) -> Self {
        match value {
            1 => TypeAplicationCommand::CHAT_INPUT,
            2 => TypeAplicationCommand::USER,
            3 => TypeAplicationCommand::MESSAGE,
            4 => TypeAplicationCommand::PRIMARY_ENTRY_POINT,
            _ => TypeAplicationCommand::CHAT_INPUT,
        }
    }
}
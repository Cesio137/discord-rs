use tokio_tungstenite::tungstenite::{protocol::CloseFrame, Utf8Bytes};

pub enum EWebsocketMessage {
    None,
    Text(Utf8Bytes),
    Close(Option<CloseFrame>)
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
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

impl TryFrom<u64> for Opcode {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Dispatch),
            1 => Ok(Opcode::Heartbeat),
            2 => Ok(Opcode::Identify),
            3 => Ok(Opcode::StatusUpdate),
            4 => Ok(Opcode::VoiceStateUpdate),
            6 => Ok(Opcode::Resume),
            7 => Ok(Opcode::Reconnect),
            8 => Ok(Opcode::RequestGuildMembers),
            9 => Ok(Opcode::InvalidSession),
            10 => Ok(Opcode::Hello),
            11 => Ok(Opcode::HeartbeatAck),
            _ => Err(()),
        }
    }
}


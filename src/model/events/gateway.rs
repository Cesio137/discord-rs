use serde_repr::{Deserialize_repr, Serialize_repr};

/*TYPES*/
#[derive(Debug, Serialize_repr, Deserialize_repr, Copy, Clone)]
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
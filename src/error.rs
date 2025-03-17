use tokio_tungstenite::tungstenite::{Error as TkError, protocol::{CloseFrame, frame::coding::CloseCode}};

#[derive(Debug)]
pub enum Error {
    ConnectionClosed(Option<CloseFrame>),
    GatewayError(GatewayCloseCode),
    WebsocketError(tokio_tungstenite::tungstenite::Error),
}

#[derive(Debug)]
pub struct GatewayCloseCode {
    pub code: u16,
    pub description: String,
    pub explanation: String,
    pub reconnect: bool,
}

impl GatewayCloseCode {
    pub fn new(code: u16) -> Self {
        let (description, explanation, reconnect) = match code {
            4000 => (
                String::from("Unknown error"),
                String::from("We're not sure what went wrong. Try reconnecting?"),
                true,
            ),
            4001 => (
                String::from("Unknown opcode"),
                String::from("You sent an invalid opcode."),
                true,
            ),
            4002 => (
                String::from("Decode error"),
                String::from("You sent an invalid payload for the opcode."),
                true,
            ),
            4003 => (
                String::from("Not authenticated"),
                String::from("You sent a payload prior to identifying."),
                true,
            ),
            4004 => (
                String::from("Authentication failed"),
                String::from("The account token sent with your identify payload is incorrect."),
                false,
            ),
            4005 => (
                String::from("Already authenticated"),
                String::from("You sent more than one identify payload. Stahp."),
                true,
            ),
            4007 => (
                String::from("Invalid seq"),
                String::from("The seq sent when resuming the session was invalid."),
                true,
            ),
            4008 => (
                String::from("Rate limited"),
                String::from("You're sending payloads too quickly. Slow it down!"),
                true,
            ),
            4009 => (
                String::from("Session timed out"),
                String::from("Your session timed out. Reconnect and re-identify."),
                true,
            ),
            4010 => (
                String::from("Invalid shard"),
                String::from("You sent an invalid shard when identifying."),
                false,
            ),
            4011 => (
                String::from("Sharding required"),
                String::from("The session would have too many shards. You are required to shard your connection."),
                false,
            ),
            4012 => (
                String::from("Invalid API version"),
                String::from("You sent an invalid version for the gateway."),
                false,
            ),
            4013 => (
                String::from("Invalid intent(s)"),
                String::from("You sent intent(s) that you have not been approved for."),
                false,
            ),
            4014 => (
                String::from("Disallowed intent(s)"),
                String::from("You sent intent(s) that you have not been approved for."),
                false,
            ),
            _ => (
                String::from("Unknown error"),
                String::from("An unknown error occurred."),
                true,
            ),
        };
        
        Self {
            code,
            description,
            explanation,
            reconnect,
        }
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for Error {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Error::WebsocketError(err)
    }
}

impl From<CloseFrame> for Error {
    fn from(err: CloseFrame) -> Self {
        let error_code = match err.code {
            CloseCode::Library(code) => code,
            _ => {
                let close_code = GatewayCloseCode {
                    code: 0,
                    description: String::from(""),
                    explanation: String::from(""),
                    reconnect: false,
                };
                return Error::GatewayError(close_code);
            }
        };

        let close_code = GatewayCloseCode::new(error_code);
        Error::GatewayError(close_code)
    }
}
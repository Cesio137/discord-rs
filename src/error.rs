/*use serde_json;
use tokio_tungstenite;
use tungstenite::protocol::CloseFrame;

#[derive(Debug)]
pub enum GatewayError {
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    SerdeError(serde_json::Error),
}

impl From<tungstenite::Error> for GatewayError {
    fn from(error: tungstenite::error::Error) -> Self {
        GatewayError::WebsocketError(error.into())
    }
}

impl From<serde_json::Error> for GatewayError {
    fn from(error: serde_json::Error) -> Self {
        GatewayError::SerdeError(error.into())
    }
}*/

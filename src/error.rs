use serde_json;
use tokio_tungstenite;

#[derive(Debug)]
pub enum GatewayError {
    WebSocketError(tokio_tungstenite::tungstenite::Error),
    //SerdeError(serde_json::Error),
    ReconnectRequired,
    InvalidSession,
}

impl From<tungstenite::Error> for GatewayError {
    fn from(error: tungstenite::error::Error) -> Self {
        GatewayError::WebSocketError(error.into())
    }
}

/*impl From<serde_json::Error> for GatewayError {
    fn from(error: serde_json::Error) -> Self {
        GatewayError::SerdeError(error.into())
    }
}*/

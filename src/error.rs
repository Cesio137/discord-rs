use tokio_tungstenite;
use serde_json;

#[derive(Debug)]
pub enum Error {
    WebSocketError(tokio_tungstenite::tungstenite::Error),
    SerdeError(serde_json::Error),
}

impl From<tungstenite::Error> for Error {
    fn from(error: tungstenite::error::Error) -> Self {
        Error::WebSocketError(error.into())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeError(error.into())
    }
}
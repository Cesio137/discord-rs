use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Error, connect_async};
use tungstenite::{Message, protocol::CloseFrame};
use futures_util::{SinkExt, StreamExt};
use crate::gateway::enums::EWebsocketMessage;

type WebsocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Websocket {
    stream: WebsocketStream,
    is_open: bool,
}

impl Websocket {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let ws_stream = connect_async(url).await?.0;
        Ok(Self {
            stream: ws_stream,
            is_open: true,
        })
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub async fn send(&mut self, message: String) -> Result<(), Error> {
        self.stream.send(Message::text(message)).await?;
        Ok(())
    }

    pub async fn pool(&mut self) -> Result<EWebsocketMessage, Error> {
        let message = match self.stream.next().await {
            None => return Ok(EWebsocketMessage::None),
            Some(result) => {
                match result {
                    Ok(msg) => {msg}
                    Err(err) => {return Err(err);}
                }
            }
        };

        match message {
            Message::Text(utf8_bytes) => {
                if utf8_bytes.is_empty() {return Ok(EWebsocketMessage::None)}
                return Ok(EWebsocketMessage::Text(utf8_bytes))
            }
            Message::Binary(_) => {return Ok(EWebsocketMessage::None)}
            Message::Close(msg) => {
                self.is_open = false;
                self.close(msg.clone()).await?;
                return Ok(EWebsocketMessage::Close(msg));
            }
            _ => {}
        }

        Ok(EWebsocketMessage::None)
    }

    pub async fn close(&mut self, msg: Option<CloseFrame>) -> Result<(), Error> {
        self.stream.close(msg).await?;
        Ok(())
    }
}
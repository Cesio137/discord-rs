use crate::config::WS_URL;
use crate::enums::{gateway, gateway::Opcode};
use futures_util::{future::try_join_all, SinkExt, StreamExt};
use serde_json::json;
use std::{cell::RefCell, str::FromStr, sync::Arc, time::Duration};
use tokio::{task::JoinHandle, net::TcpStream, sync::Mutex as TokioMutex, time::sleep};
use tokio_tungstenite::{
    connect_async, tungstenite::Error, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};
use tungstenite::protocol::CloseFrame;
use crate::error::GatewayError;

type ArcGateway = Arc<TokioMutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>;
type ArcBool = Arc<TokioMutex<bool>>;

pub struct Websocket {
    stream: ArcGateway,
    handlers: RefCell<Vec<JoinHandle<()>>>,
    is_open: ArcBool,
    ack_received: ArcBool,
}

impl Websocket {
    /*PUBLIC*/
    pub async fn new() -> Result<Self, Error> {
        let stream = Arc::new(TokioMutex::new(connect_async(WS_URL).await?.0));
        Ok(Self {
            stream,
            handlers: RefCell::new(Vec::new()),
            is_open: Arc::new(TokioMutex::new(true)),
            ack_received: Arc::new(TokioMutex::new(false)),
        })
    }

    pub async fn listen(&self, token: &str) -> Result<(), GatewayError> {
        let identify = json!({
            "op": 2,
            "d": {
                "token": token,
                "intents": 513,
                "properties": {
                    "os": &sys_info::os_type().unwrap_or(String::from("unknown")),
                    "device": "discord-rs",
                    "browser": "discord-rs",
                }
            }
        })
        .to_string();
        match self.stream.lock().await.send(Message::text(identify)).await {
            Ok(_) => {}
            Err(e) => {
                if cfg!(debug_assertions) {
                    eprintln!("Error trying to send a identify payload.\n{:?}", e);
                }
            }
        }

        while let Some(message) = self.stream.lock().await.next().await {
            let payload = match message {
                Ok(data) => {
                    if data.is_empty() {
                        continue;
                    }
                    data
                }
                Err(e) => {
                    return Err(GatewayError::from(e));
                }
            };

            match payload {
                Message::Text(utf8bytes) => {
                    if utf8bytes.is_empty() {
                        continue;
                    }
                    self.events(utf8bytes.as_str()).await?;
                }
                Message::Binary(bytes) => {
                    println!("bytes: {}", bytes.len());
                    continue;
                }
                Message::Close(_) => {
                    *self.is_open.lock().await = false;
                    // Trigger close event
                    return Ok(());
                }
                _ => {
                    continue;
                }
            }
        }

        Ok(())
    }

    pub async fn async_send(&self, message: &str) -> Result<(), Error> {
        self.stream.lock().await.send(Message::text(message)).await?;
        Ok(())
    }

    /*PRIVATE*/
    async fn events(&self, message: &str) -> Result<(), GatewayError> {
        let parse = match serde_json::Value::from_str(message) {
            Ok(value) => value,
            Err(e) => {
                if cfg!(debug_assertions) {
                    eprintln!("Error trying to parse Discord API message.\n{:?}", e);
                }
                return Ok(())
            }
        };

        let opcode = match parse["op"].as_u64() {
            None => {
                return Ok(())
            }
            Some(op) => match gateway::Opcode::try_from(op) {
                Ok(op) => op,
                Err(e) => {
                    if cfg!(debug_assertions) {
                        eprintln!("Error trying to get opcode field from Discord API message.\n{:?}", e);
                    }
                    return Ok(())
                }
            },
        };

        match opcode {
            Opcode::Dispatch => {}
            Opcode::Heartbeat => {
                let heartbeat_payload = json!({
                    "op": 1,
                    "d": null
                });
                match self.stream.lock().await.send(Message::text(heartbeat_payload.to_string())).await {
                    Ok(_) => {}
                    Err(e) => {
                        if cfg!(debug_assertions) {
                            eprintln!("Error trying to send a hertbeat.\n{:?}", e);
                        }
                    }
                }
            }
            Opcode::Reconnect => {
                if cfg!(debug_assertions) {
                    eprintln!("Need to reconnect.");
                }
            }
            Opcode::InvalidSession => {
                if cfg!(debug_assertions) {
                    eprintln!("Invalid session.");
                }
            }
            Opcode::Hello => {
                let interval = match parse["d"].as_object() {
                    None => 40000,
                    Some(d) => d["heartbeat_interval"].as_u64().unwrap_or(40000),
                };
                let stream_clone = Arc::clone(&self.stream);
                let is_open_clone = Arc::clone(&self.is_open);
                let ack_received_clone = Arc::clone(&self.ack_received);
                let joinhandle = tokio::spawn(async move {
                    Websocket::heartbeat(stream_clone, is_open_clone, ack_received_clone, interval)
                        .await;
                });
                self.handlers.borrow_mut().push(joinhandle);
            }
            Opcode::HeartbeatAck => {
                *self.ack_received.lock().await = true;
            }
            _ => {}
        }
        
        Ok(())
    }

    async fn heartbeat(stream: ArcGateway, is_open: ArcBool, ack_received: ArcBool, interval: u64) {
        let heartbeat_payload = json!({
            "op": 1,
            "d": null
        }).to_string();
        while *is_open.lock().await {
            let _ = stream.lock().await.send(Message::text(&heartbeat_payload)).await;
            sleep(Duration::from_millis(interval)).await;
            if !*ack_received.lock().await {
                if cfg!(debug_assertions) {
                    eprintln!("Heartbeat acknowledgment not received! Closing connection.");
                }
                let _ = stream.lock().await.close(Some(CloseFrame {
                        code: tungstenite::protocol::frame::coding::CloseCode::Away,
                        reason: "Heartbeat acknowledgment not received.".into(),
                    })).await;
                *is_open.lock().await = false;
                break;
            }
            *ack_received.lock().await = false;
        }
    }
}

impl Drop for Websocket {
    fn drop(&mut self) {
        if self.handlers.borrow().len() > 0 {
            let _ = try_join_all(self.handlers.borrow_mut().drain(..));
        }
    }
}

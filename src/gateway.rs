pub mod websocket;
pub mod enums;

use crate::config::WS_URL;
//use crate::enums::{gateway, gateway::Opcode};
use crate::error::GatewayError;
use futures_util::{future::try_join_all, SinkExt, StreamExt};
use serde_json::{json, Value};
use std::{cell::RefCell, str::FromStr, sync::Arc, time::Duration};
use tokio::{
    net::TcpStream,
    sync::Mutex as TokioMutex,
    task::JoinHandle,
    time::sleep,
};
use tokio_tungstenite::{
    connect_async, tungstenite::Error, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};
use tungstenite::protocol::CloseFrame;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::frame::coding::CloseCode::Away;
use crate::gateway::enums::{EMessage, Opcode};
use crate::gateway::websocket::Websocket;
//use crate::error::GatewayError;

type ArcWebsocket = Arc<TokioMutex<Websocket>>;
type ArcBool = Arc<TokioMutex<bool>>;

pub struct Gateway {
    websocket: ArcWebsocket,
    handler_tasks: RefCell<Vec<JoinHandle<()>>>,
    is_ack_received: ArcBool,
}

impl Drop for Gateway {
    fn drop(&mut self) {
        if self.handler_tasks.borrow().len() > 0 {
            let _ = try_join_all(self.handler_tasks.borrow_mut().drain(..));
        }
    }
}

impl Gateway {
    async fn new() -> Result<Self, Error> {
        let ws = Arc::new(TokioMutex::new(Websocket::new(WS_URL).await?));
        Ok(Self {
            websocket: ws,
            handler_tasks: RefCell::new(Vec::new()),
            is_ack_received: Arc::new(TokioMutex::new(false)),
        })
    }
    
    pub async fn listen(&mut self, bot_token: &str) -> Result<(), Error>  {
        let identify_payload = json!({
            "op": 2,
            "d": {
                "token": bot_token,
                "intents": 513,
                "properties": {
                    "os": &sys_info::os_type().unwrap_or(String::from("unknown")),
                    "device": "discord-rs",
                    "browser": "discord-rs",
                }
            }
        }).to_string();
        self.websocket.lock().await.send(EMessage::Text(identify_payload)).await?;
        self.websocket.lock().await.listen().await?;
        Ok(())
    }

    async fn message_recvd(&mut self, message: EMessage) -> Result<(), GatewayError> {
        match message {
            EMessage::Text(text) => {
                self.handle_events(text.as_str()).await?;
            }
            EMessage::Binary(_) => {}
        }
        Ok(())
    }
    
    async fn handle_events(&mut self, data: &str) -> Result<(), GatewayError>  {
        let parse = match serde_json::Value::from_str(data) {
            Ok(str) => str,
            Err(err) => return Err(GatewayError::from(err))
        };

        let opcode = match parse["op"].as_u64() {
            None => return Ok(()),
            Some(op) => match enums::Opcode::try_from(op) {
                Ok(op) => op,
                Err(e) => {
                    if cfg!(debug_assertions) {
                        eprintln!("Error getting opcode from Discord API message: {:?}", e);
                    }
                    return Ok(());
                }
            },
        };

        match opcode {
            Opcode::Dispatch => {}
            Opcode::Heartbeat => {
                let heartbeat_payload = json!({
                    "op": 1,
                    "d": null
                }).to_string();
                self.websocket.lock().await.send(EMessage::Text(heartbeat_payload)).await?;
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
                let interval: u64 = match parse["d"].as_object() {
                    None => 40000,
                    Some(d) => d["heartbeat_interval"].as_u64().unwrap_or(40000),
                };

                let ws_clone = Arc::clone(&self.websocket);
                let is_ack_received_clone = Arc::clone(&self.is_ack_received);

                let task_handle = tokio::spawn(async move {
                    Gateway::heartbeat(ws_clone, is_ack_received_clone, interval).await;
                });

                self.handler_tasks.borrow_mut().push(task_handle);
            }
            Opcode::HeartbeatAck => {
                *self.is_ack_received.lock().await = true;
            }
            _ => {}
        }
        
        Ok(())
    }
    
    async fn heartbeat(websocket: ArcWebsocket, is_ack_received: ArcBool, interval: u64) {
        let heartbeat_payload = json!({
            "op": 1,
            "d": null
        }).to_string();
        
        while websocket.lock().await.is_open() {
            let _ = websocket.lock().await.send(EMessage::Text(heartbeat_payload.clone())).await;

            sleep(Duration::from_millis(interval)).await;

            if !*is_ack_received.lock().await {
                if cfg!(debug_assertions) {
                    eprintln!("Heartbeat acknowledgment not received! Closing connection.");
                }

                let _ = websocket.lock().await.close(Some(CloseFrame {
                    code: tungstenite::protocol::frame::coding::CloseCode::Away,
                    reason: "Heartbeat acknowledgment not received.".into(),
                })).await;
                let _ = websocket.lock().await.close(Some(CloseFrame {
                    code: CloseCode::Away,
                    reason: "Heartbeat acknowledgment not received.".into(),
                })).await;
                break;
            }

            *is_ack_received.lock().await = false;
        }
    }
}
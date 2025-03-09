pub mod websocket;
pub mod enums;

use crate::config::WS_URL;
use crate::gateway::{enums::{EClientEvent, EWebsocketMessage, Opcode}, websocket::Websocket};
use enums::EGatewayEvent;
use serde_json::{json, Value};
use std::{sync::Arc, str::FromStr, time::Duration};
use tokio::{task::JoinHandle, time::sleep, sync::Mutex};
use tokio_tungstenite::tungstenite::Error;
use tungstenite::protocol::{CloseFrame, frame::coding::CloseCode};

type ArcWebsocket = Arc<Mutex<Websocket>>;
type ArcBool = Arc<Mutex<bool>>;

pub struct Gateway {
    websocket: ArcWebsocket,
    handler_task: Option<JoinHandle<()>>,
    is_ack_received: ArcBool
}

impl Drop for Gateway {
    fn drop(&mut self) {
        match &self.handler_task {
            None => {}
            Some(task) => {
                task.abort();
            }
        }
    }
}

impl Gateway {
    pub async fn new() -> Result<Self, Error> {
        let ws = Arc::new(Mutex::new(Websocket::new(WS_URL).await?));
        Ok(Self {
            websocket: ws,
            handler_task: None,
            is_ack_received: Arc::new(Mutex::new(false)),
        })
    }

    pub async fn identify(&self, bot_token: &str) -> Result<(), Error> {
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
        self.websocket.lock().await.send(identify_payload).await?;
        Ok(())
    }
    
    pub async fn pool(&mut self) -> Result<EGatewayEvent, Error>  {
        let data = match self.websocket.lock().await.pool().await? {
            EWebsocketMessage::None => return Ok(EGatewayEvent::Dispatch(EClientEvent::None)),
            EWebsocketMessage::Text(utf8_bytes) => utf8_bytes.to_string(),
            EWebsocketMessage::Close(close_frame) => return Ok(EGatewayEvent::Close(close_frame)),
        };

        Ok(self.handle_events(&data).await?)
    }

    pub async fn close(&mut self, msg: Option<CloseFrame>) -> Result<(), Error>  {
        self.websocket.lock().await.close(msg).await?;
        Ok(())
    }
    
    async fn handle_events(&mut self, data: &str) -> Result<EGatewayEvent, Error>  {
        let parse = match Value::from_str(data) {
            Ok(str) => str,
            Err(err) => {
                if cfg!(debug_assertions) {
                    eprintln!("Error trying to parse json data.\n{}", err);
                }
                return Ok(EGatewayEvent::Dispatch(EClientEvent::None))
            }
        };

        let opcode = match parse["op"].as_u64() {
            None => return Ok(EGatewayEvent::Dispatch(EClientEvent::None)),
            Some(op) => match enums::Opcode::try_from(op) {
                Ok(op) => op,
                Err(_) => {
                    if cfg!(debug_assertions) {
                        eprintln!("Error trying to get 'op' field from json data.");
                    }
                    return Ok(EGatewayEvent::Dispatch(EClientEvent::None));
                }
            },
        };

        match opcode {
            Opcode::Dispatch => {
                              
            }
            Opcode::Heartbeat => {
                let heartbeat_payload = json!({
                    "op": 1,
                    "d": null
                }).to_string();
                self.websocket.lock().await.send(heartbeat_payload).await?;
            }
            Opcode::Reconnect => {
                if cfg!(debug_assertions) {
                    eprintln!("Need to reconnect.");
                }
                
                return Ok(EGatewayEvent::ReconnectRequired);
            }
            Opcode::InvalidSession => {
                if cfg!(debug_assertions) {
                    eprintln!("Invalid session.");
                }

                let d = match parse["d"].as_bool() {
                    None => {false}
                    Some(t) => {t}
                };
                return Ok(EGatewayEvent::InvalidSession(d));
            }
            Opcode::Hello => {
                match &self.handler_task {
                    None => {}
                    Some(task) => {
                        task.abort();
                    }
                }
                
                let interval: u64 = match parse["d"].as_object() {
                    None => 40000,
                    Some(d) => d["heartbeat_interval"].as_u64().unwrap_or(40000),
                };
                let ws_clone = Arc::clone(&self.websocket);
                let is_ack_received_clone = Arc::clone(&self.is_ack_received);
                self.handler_task = tokio::spawn(async move {
                    Gateway::heartbeat(ws_clone, is_ack_received_clone, interval).await;
                }).into();
            }
            Opcode::HeartbeatAck => {
                *self.is_ack_received.lock().await = true;
            }
            _ => {}
        }
        
        Ok(EGatewayEvent::Dispatch(EClientEvent::None))
    }

    async fn heartbeat(websocket: Arc<Mutex<Websocket>>, is_ack_received: Arc<Mutex<bool>>, interval: u64) {
        let heartbeat_payload = json!({
            "op": 1,
            "d": null
        }).to_string();

        loop {
            if !websocket.lock().await.is_open() {
                return;
            }
            
            let _ = match websocket.lock().await.send(heartbeat_payload.clone()).await {
                Err(_) => return,
                _ => {}
            };
            sleep(Duration::from_millis(interval)).await;
            
            if !*is_ack_received.lock().await {
                if cfg!(debug_assertions) {
                    eprintln!("Heartbeat acknowledgment not received! Closing connection.");
                }

                if !websocket.lock().await.is_open() {
                    return;
                }
                
                let _ = websocket.lock().await.close(Some(CloseFrame {
                    code: tungstenite::protocol::frame::coding::CloseCode::Away,
                    reason: "Heartbeat acknowledgment not received.".into(),
                })).await;
                let _ = websocket.lock().await.close(Some(CloseFrame {
                    code: CloseCode::Away,
                    reason: "Heartbeat acknowledgment not received.".into(),
                })).await;
                return;
            }

            *is_ack_received.lock().await = false;
        }
    }
}
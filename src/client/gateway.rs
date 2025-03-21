use crate::config::WS_URL;
use super::websocket::{Websocket, EWebsocketMessage};
use serde_json::{json, Value};
use std::{sync::Arc, str::FromStr, time::Duration};
use colored::Colorize;
use tokio::{task::JoinHandle, time::sleep, sync::Mutex};
use tokio_tungstenite::tungstenite::{Error, protocol::{CloseFrame, frame::coding::CloseCode}};
use crate::error;
use crate::model::events::gateway::{GatewayOpcode, Identify, Payload, Ready, ReceivedEvent};

/*TYPES*/
type ArcWebsocket = Arc<Mutex<Websocket>>;
type ArcBool = Arc<Mutex<bool>>;

/*EVENTS*/
pub enum EGatewayEvent {
    Dispatch(ReceivedEvent),
    ReconnectRequired,
    InvalidSession(bool),
    Close(CloseFrame)
}

/*STRUCT*/
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
        let identify = Identify {
            token: format!("Bot {}", bot_token),
            ..std::default::Default::default()
        };
        let payload = Payload {
            op: GatewayOpcode::Identify,
            d: serde_json::to_value(identify).unwrap_or_else(|err| {
                eprintln!("{}", err);
                Value::Null
            }).into(),
            ..std::default::Default::default()
        };
        let json = serde_json::to_string(&payload).unwrap_or_else(|err| {
            eprintln!("{}", err);
            String::new()
        });
        self.websocket.lock().await.send(json).await?;
        Ok(())
    }

    pub async fn pool(&mut self) -> Result<EGatewayEvent, error::Error>  {
        let data = match self.websocket.lock().await.pool().await? {
            EWebsocketMessage::None => return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None)),
            EWebsocketMessage::Text(utf8_bytes) => utf8_bytes.to_string(),
            EWebsocketMessage::Close(close_frame) => {
                let frame = match close_frame {
                    None => return Err(error::Error::ConnectionClosed(None)),
                    Some(frame) => frame
                };

                let frame_code = match frame.code {
                    CloseCode::Library(code) => code,
                    _ => {return Err(error::Error::from(frame))}
                };

                match frame_code {
                    4000 => return Ok(EGatewayEvent::Close(frame)),
                    4001 => return Ok(EGatewayEvent::Close(frame)),
                    4002 => return Ok(EGatewayEvent::Close(frame)),
                    4003 => return Ok(EGatewayEvent::Close(frame)),
                    4004 => return Err(error::Error::from(frame)),
                    4005 => return Ok(EGatewayEvent::Close(frame)),
                    4006 => return Ok(EGatewayEvent::Close(frame)),
                    4008 => return Ok(EGatewayEvent::Close(frame)),
                    4009 => return Ok(EGatewayEvent::Close(frame)),
                    4010 => return Err(error::Error::from(frame)),
                    4011 => return Err(error::Error::from(frame)),
                    4012 => return Err(error::Error::from(frame)),
                    4013 => return Err(error::Error::from(frame)),
                    4014 => return Err(error::Error::from(frame)),
                    _ => {}
                }
                return Err(error::Error::ConnectionClosed(frame.into()));
            },
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
                eprintln!("Error trying to parse json data.\n{}", err);
                return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None))
            }
        };
        

        let opcode = match parse["op"].as_u64() {
            None => return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None)),
            Some(op) => { 
                match op { 
                    0 => GatewayOpcode::Dispatch,
                    1 => GatewayOpcode::Heartbeat,
                    2 => GatewayOpcode::Identify,
                    3 => GatewayOpcode::StatusUpdate,
                    4 => GatewayOpcode::VoiceStateUpdate,
                    6 => GatewayOpcode::Resume,
                    7 => GatewayOpcode::Reconnect,
                    8 => GatewayOpcode::RequestGuildMembers,
                    9 => GatewayOpcode::InvalidSession,
                    10 => GatewayOpcode::Hello,
                    11 => GatewayOpcode::HeartbeatAck,
                    _ => {
                        eprintln!("Error trying to parse opcode from json data.");
                        return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None))
                    }
                }
            }
        };

        match opcode {
            GatewayOpcode::Dispatch => {
                let event_name = match parse["t"].as_str() {
                    None => return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None)),
                    Some(event) => event,
                };
                let data = match parse["data"].as_str() {
                    None => return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None)),
                    Some(data) => data,
                };
                
                match event_name { 
                    "READY" => {
                        let ready: Ready = match serde_json::from_str(data) {
                            Ok(ready) => ready,
                            Err(err) => {
                                eprintln!("Error trying to parse ready data.\n{}", err);
                                return Ok(EGatewayEvent::Dispatch(ReceivedEvent::None))
                            }
                        };
                        return Ok(EGatewayEvent::Dispatch(ReceivedEvent::Ready(ready)))
                    }
                    _ => todo!()
                }
            }
            GatewayOpcode::Heartbeat => {
                let heartbeat_payload = json!({
                    "op": 1,
                    "d": null
                }).to_string();
                self.websocket.lock().await.send(heartbeat_payload).await?;
            }
            GatewayOpcode::Reconnect => {
                println!("{}", "Need to reconnect.".yellow());

                return Ok(EGatewayEvent::ReconnectRequired);
            }
            GatewayOpcode::InvalidSession => {
                println!("{}", "Invalid session.".yellow());

                let d = match parse["d"].as_bool() {
                    None => {false}
                    Some(t) => {t}
                };
                return Ok(EGatewayEvent::InvalidSession(d));
            }
            GatewayOpcode::Hello => {
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
            GatewayOpcode::HeartbeatAck => {
                *self.is_ack_received.lock().await = true;
            }
            _ => {}
        }

        Ok(EGatewayEvent::Dispatch(ReceivedEvent::None))
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
                println!("{}", "Heartbeat acknowledgment not received! Closing connection.".yellow());

                if !websocket.lock().await.is_open() {
                    return;
                }

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
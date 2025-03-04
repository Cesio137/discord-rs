use crate::config::WS_URL;
use crate::enums::{gateway, gateway::Opcode};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::{net::TcpStream, sync::Mutex as TokioMutex, time::sleep};
use tokio_tungstenite::{
    connect_async, tungstenite::error::Error, tungstenite::Message, MaybeTlsStream, WebSocketStream,
};
use tungstenite::protocol::CloseFrame;

type MutexGateway = Arc<TokioMutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>;
type MutexBool = Arc<TokioMutex<bool>>;

pub struct Websocket {
    stream: MutexGateway,
    is_open: MutexBool,
    ack_received: MutexBool,
}

impl Websocket {
    /*PUBLIC*/
    pub async fn new() -> Result<Self, Error> {
        let stream = Arc::new(TokioMutex::new(connect_async(WS_URL).await?.0));
        Ok(Self {
            stream,
            is_open: Arc::new(TokioMutex::new(true)),
            ack_received: Arc::new(TokioMutex::new(false)),
        })
    }

    pub async fn listen(&self, token: &str) -> Result<(), Error> {
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
        }).to_string();
        /*self.stream
            .lock()
            .await
            .send(Message::text(identify))
            .await?;*/

        while let Some(message) = self.stream.lock().await.next().await {
            let payload = match message {
                Ok(data) => {
                    if data.is_empty() {
                        continue;
                    }
                    data
                }
                Err(e) => {
                    return Err(e);
                }
            };

            match payload {
                Message::Text(utf8bytes) => {
                    if utf8bytes.is_empty() {
                        continue;
                    }
                    self.events(utf8bytes.as_str()).await;
                }
                Message::Binary(bytes) => {
                    println!("bytes: {}", bytes.len());
                    continue;
                }
                Message::Close(frame) => {
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
        self.stream
            .lock()
            .await
            .send(Message::text(message))
            .await?;
        Ok(())
    }

    /*PRIVATE*/
    async fn events(&self, message: &str) {
        let parse = match serde_json::Value::from_str(message) {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Failed to parse message");
                return;
            }
        };

        let opcode = match parse["op"].as_u64() {
            None => {
                return;
            }
            Some(op) => match gateway::Opcode::try_from(op) {
                Ok(op) => op,
                Err(_) => {
                    return;
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
                        eprintln!("Failed to send heartbeat!\n{}", e);
                    }
                }
            }
            Opcode::Identify => {}
            Opcode::StatusUpdate => {}
            Opcode::VoiceStateUpdate => {}
            Opcode::Resume => {}
            Opcode::Reconnect => {}
            Opcode::RequestGuildMembers => {}
            Opcode::InvalidSession => {
                eprintln!("Invalid session.");
            }
            Opcode::Hello => {
                let heartbeat_interval = match parse["d"].as_object() {
                    None => 40000,
                    Some(d) => d["heartbeat_interval"].as_u64().unwrap_or(40000),
                };
                let stream_clone = Arc::clone(&self.stream);
                let is_open_clone = Arc::clone(&self.is_open);
                let ack_received_clone = Arc::clone(&self.ack_received);
                tokio::spawn(async move {
                    heartbeat(
                        stream_clone,
                        is_open_clone,
                        ack_received_clone,
                        heartbeat_interval,
                    )
                    .await;
                });
            }
            Opcode::HeartbeatAck => {
                *self.ack_received.lock().await = true;
            }
        }
    }
}

async fn heartbeat(
    stream: MutexGateway,
    is_open: MutexBool,
    ack_received: MutexBool,
    interval: u64,
) {
    let heartbeat_payload = json!({
        "op": 1,
        "d": null
    })
    .to_string();
    while *is_open.lock().await {
        stream
            .lock()
            .await
            .send(Message::text(&heartbeat_payload))
            .await;
        sleep(Duration::from_millis(interval)).await;
        if !*ack_received.lock().await {
            eprintln!("Heartbeat acknowledgment not received! Closing connection.");
            stream
                .lock()
                .await
                .close(Some(CloseFrame {
                    code: tungstenite::protocol::frame::coding::CloseCode::Away,
                    reason: "Heartbeat acknowledgment not received.".into(),
                }))
                .await;
            *is_open.lock().await = false;
            break;
        }
        *ack_received.lock().await = false;
    }
}

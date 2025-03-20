pub mod gateway;
pub mod websocket;

use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use crate::client::gateway::{EGatewayEvent, Gateway};
use crate::error;
use crate::error::GatewayCloseCode;
use crate::model::events::gateway::ReceivedEvent;
use crate::utils::options::Options;



pub struct Client {
    bot_token: String,
    client_options: Options,
    gateway: Gateway
}

impl Client {
    pub async fn new(bot_token: String, client_options: Options) -> Result<Client, Error> {
        let gateway = Gateway::new().await?;
        Ok(Self {
            bot_token,
            client_options,
            gateway,
        })
    }

    pub async fn login(&self) -> Result<(), Error> {
        self.gateway.identify(&self.bot_token).await?;
        Ok(())
    }

    pub async fn pool(&mut self) -> Result<ReceivedEvent, error::Error> {
        match self.gateway.pool().await? {
            EGatewayEvent::Dispatch(event) => return Ok(event),
            EGatewayEvent::Close(close_frame) => {
                let code = match close_frame.code {
                    CloseCode::Library(code) => code,
                    _ => {return Err(error::Error::ConnectionClosed(close_frame.into()))}
                };
                let close_code = GatewayCloseCode::new(code);
                eprintln!("Close code: {}\nDescription: {}\nExplanation: {}\nReconnect: {}\n",
                          close_code.code,
                          close_code.explanation,
                          close_code.description,
                          close_code.reconnect
                );
                self.reconnect().await?;
            },
            _ => {}
        }

        Ok(ReceivedEvent::None)
    }

    async fn reconnect(&mut self) -> Result<(), Error> {
        let _ = self.gateway.close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "".into(),
        })).await;

        self.gateway = Gateway::new().await?;
        self.gateway.identify(&self.bot_token).await?;
        Ok(())
    }
}
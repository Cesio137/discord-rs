use tokio_tungstenite::tungstenite::{protocol::{frame::coding::CloseCode, CloseFrame}, Error};
use crate::error::GatewayCloseCode;
use crate::gateway::{
    Gateway,
    events::{
        EClientEvent,
        EGatewayEvent
    }
};
use crate::utils::options::*;

mod config;
mod error;
mod gateway;
mod utils;
mod handlers;
mod resources;
mod interactions;
mod events;
pub mod model;
pub mod internal;

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

    pub async fn pool(&mut self) -> Result<EClientEvent, error::Error> {
        match self.gateway.pool().await? {
            EGatewayEvent::Dispatch(client_event) => return Ok(client_event),
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

        Ok(EClientEvent::None)
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

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use std::env;
    use crate::Client;
    use crate::error::Error;
    use crate::utils::options::Options;

    #[tokio::test]
    async fn it_works() {
        dotenv().ok();
        let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not defined in file .env");
        println!("Bot token: {}", bot_token);
        let mut client = match Client::new(bot_token, Options::default()).await {
            Ok(client) => client,
            Err(err) => panic!("{:?}", err)
        };
        
        match client.login().await {
            Err(err) => panic!("{}", err),
            _ => {}
        }

        loop {
            let events = match client.pool().await {
                Ok(event) => event,
                Err(err) => {
                    match err {
                        Error::ConnectionClosed(error) => {
                            panic!("{:?}", error)
                        }
                        Error::GatewayError(error) => {
                            panic!("{}", error.code)
                        }
                        Error::WebsocketError(error) => {
                            panic!("{}", error)
                        }
                    }
                },
            };
            println!("Event name: {:?}", events);
        }
    }
}

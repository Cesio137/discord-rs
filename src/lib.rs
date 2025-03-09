use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::Error;
use crate::gateway::enums::EClientEvent;
use crate::gateway::Gateway;
use crate::utils::options::*;

mod config;
mod error;
mod gateway;
mod utils;

pub struct Client {
    bot_token: String,
    client_options: Options,
    gateway: Gateway,
    should_reconnect: bool
}

impl Client {
    pub async fn new(bot_token: String, client_options: Options) -> Result<Client, Error> {
        let gateway = Gateway::new().await?;
        Ok(Self {
            bot_token,
            client_options,
            gateway,
            should_reconnect: false
        })
    }

    pub async fn login(&self) -> Result<(), Error> {
        self.gateway.identify(&self.bot_token).await?;
        Ok(())
    }

    pub async fn pool(&mut self) -> Result<EClientEvent, Error> {
        match self.gateway.pool().await? {
            gateway::enums::EGatewayEvent::Dispatch(client_event) => return Ok(client_event),
            gateway::enums::EGatewayEvent::ReconnectRequired => {
                self.should_reconnect = true;
            },
            gateway::enums::EGatewayEvent::InvalidSession(d) => {
                self.should_reconnect = d;
            },
            gateway::enums::EGatewayEvent::Close(close_frame) => {
                if cfg!(debug_assertions) {
                    match close_frame {
                        Some(frame) => println!("Connection closed.\n{}", frame),
                        None => println!("Connection closed."),
                    }
                }
                if !self.should_reconnect {
                    return Err(Error::ConnectionClosed);
                }
                self.reconnect().await?;
                self.should_reconnect = false;
            },
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
            Err(err) => panic!("{:?}", err),
            _ => {}
        }

        loop {
            let events = match client.pool().await {
                Ok(event) => event,
                Err(err) => panic!("{}", err),
            };
            println!("Event name: {:?}", events);
        }
    }
}

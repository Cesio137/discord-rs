use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::{protocol::{frame::coding::CloseCode, CloseFrame}, Error};
use crate::error::GatewayCloseCode;
use crate::utils::options::*;
pub mod config;
pub mod error;
pub mod model;
pub mod internal;
pub mod utils;
pub mod client;

#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use std::env;
    use serde_json::json;
    use crate::client::Client;
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

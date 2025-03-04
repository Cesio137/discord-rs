use crate::error::{GatewayError};
use crate::gateway::Websocket;
use crate::utils::options::*;

mod config;
mod enums;
mod error;
mod gateway;
mod utils;

pub struct Client {
    token: String,
    options: Options,
}

impl Client {
    pub fn new(token: String, options: Options) -> Self {
        Self {
            token,
            options,
        }
    }

    pub async fn login(&self) -> Result<(), GatewayError> {
        let mut should_reconnect = false;
        while !should_reconnect {
            let ws = Websocket::new().await?;
            let result = ws.listen(&self.token).await;
            match result {
                Ok(_) => { should_reconnect = true; }
                Err(e) => {
                    match e {
                        GatewayError::ReconnectRequired => { should_reconnect = false; }
                        _ => { should_reconnect = true; }
                    }
                    if should_reconnect { return Err(e); }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use crate::utils::options::Options;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn it_works() {
        dotenv().ok();
        let bot_token = env::var("BOT_TOKEN").expect("BOT_TOKEN not defined in file .env");
        println!("Bot token: {}", bot_token);
        let client = Client::new(
            bot_token,
            Options {
                intents: 32,
                ..Default::default()
            },
        );
        
        match client.login().await {
            Ok(_) => return,
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}

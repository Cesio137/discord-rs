use crate::error::{GatewayError};
use crate::gateway::websocket::Websocket;
use crate::utils::options::*;

mod config;
mod enums;
mod error;
mod gateway;
mod utils;

/*pub struct Client {
    bot_token: String,
    client_options: Options,
}

impl Client {
    pub fn new(bot_token: String, client_options: Options) -> Self {
        Self {
            bot_token,
            client_options,
        }
    }

    pub async fn login(&self) -> Result<(), GatewayError> {
        let mut is_connected = false;
        while !is_connected {
            let websocket = Websocket::new().await?;
            let connection_result = websocket.listen(&self.bot_token).await;
            match connection_result {
                Ok(_) => { is_connected = true; }
                Err(gateway_error) => {
                    match gateway_error {
                        GatewayError::ReconnectRequired => { is_connected = false; }
                        _ => { is_connected = true; }
                    }
                    if is_connected { return Err(gateway_error); }
                }
            }
        }
        Ok(())
    }
}*/

#[cfg(test)]
mod tests {
    //use crate::Client;
    use crate::utils::options::Options;
    use dotenvy::dotenv;
    use std::env;
    use crate::gateway::websocket::Websocket;
    fn teste() {
        println!("Ola, mundo!")
    }
    #[tokio::test]
    async fn it_works() {
        /*dotenv().ok();
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
        }*/
        let msg = "Ola, mundo!";
        let teste = Websocket::new("")
            .await
            .unwrap()
            .on_open(teste)
            .on_open(move || {
                println!("{}", msg);
            });
        
        teste.listen().await;
    }
}

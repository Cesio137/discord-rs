use crate::gateway::Websocket;
use crate::utils::{ options };
use crate::utils::options::*;
use crate::error::Error;

mod config;
mod utils;
mod enums;
mod error;
mod gateway;

pub struct Client {
    token: String,
    options: Options,
    onhandler: Vec<fn(&str)>,
}

impl Client {
    pub fn new(token: String, options: Options) -> Self {
        Self {
            token,
            options,
            onhandler: Vec::new(),
        }
    }

    pub async fn login(&self) -> Result<(), Error> {
        let mut ws = Websocket::new().await?;
        ws.listen(&self.token).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use crate::utils::options::Options;

    #[tokio::test]
    async fn it_works() {
        
    }
}

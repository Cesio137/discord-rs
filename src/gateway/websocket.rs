use std::pin::Pin;
use tokio::{net::TcpStream};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, tungstenite::Error, connect_async};
use tungstenite::{Message, protocol::CloseFrame};
use futures_util::{SinkExt, StreamExt};
use crate::gateway::enums::{EMessage, ERawData};

type WebsocketStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Websocket {
    stream: WebsocketStream,
    is_open: bool,
    on_open: Vec<Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>>,
    on_message: Vec<Box<dyn Fn(&ERawData) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>>,
    on_close: Vec<Box<dyn Fn(&Option<CloseFrame>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>>>>,
}

impl Websocket {
    pub async fn new(url: &str) -> Result<Self, Error> {
        let ws_stream = connect_async(url).await?.0;
        Ok(Self {
            stream: ws_stream,
            is_open: true,
            on_open: Vec::new(),
            on_message: Vec::new(),
            on_close: Vec::new(),
        })
    }

    pub fn on_open<F>(mut self, function: F) -> Self
    where
        F: Fn() -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + 'static,
    {
        self.on_open.push(Box::new(function));
        self
    }

    pub fn on_message<F>(mut self, function: F) -> Self
    where
        F: Fn(&ERawData) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + 'static,
    {
        self.on_message.push(Box::new(function));
        self
    }

    pub fn on_close<F>(mut self, function: F) -> Self
    where
        F: Fn(&Option<CloseFrame>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync>> + 'static,
    {
        self.on_close.push(Box::new(function));
        self
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub async fn send(&mut self, message: EMessage) -> Result<(), Error> {
        match message {
            EMessage::Text(text) => {
                self.stream.send(Message::text(text)).await?;
            }
            EMessage::Binary(bytes) => {
                self.stream.send(Message::Binary(bytes)).await?;
            }
        }
        Ok(())
    }

    pub async fn listen(&mut self) -> Result<(), Error>{
        self.call_on_open();

        while let Some(stream_message) = self.stream.next().await {
            let message = match stream_message {
                Ok(msg) => {msg}
                Err(err) => {return Err(err)}
            };

            match message {
                Message::Text(text) => {
                    if text.is_empty() {continue}
                    self.call_on_message(ERawData::Text(text)).await;
                }
                Message::Binary(bytes) => {
                    if bytes.is_empty() {continue}
                    self.call_on_message(ERawData::Binary(bytes)).await;
                }
                Message::Close(msg) => {
                    self.close(msg.clone()).await?;
                    self.is_open = false;
                    self.call_on_close(msg).await;
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub async fn close(&mut self, msg: Option<CloseFrame>) -> Result<(), Error> {
        self.stream.close(msg).await?;
        Ok(())
    }

    async fn call_on_open(&self) {
        if !self.on_open.is_empty() {return;}
        for func in &self.on_open {
            func().await;
        }
    }

    async fn call_on_message(&self, message: ERawData) {
        if !self.on_message.is_empty() {return;}
        for func in &self.on_message {
            func(&message).await;
        }
    }

    async fn call_on_close(&self, msg: Option<CloseFrame>) {
        if !self.on_close.is_empty() {return;}
        for func in &self.on_close {
            func(&msg).await;
        }
    }
}
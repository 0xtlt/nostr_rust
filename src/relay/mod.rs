use crate::utils::event::EventSigned;
use crate::utils::req::{Req, ReqFilter};
use crate::Message;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use thiserror::Error;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use url::Url;

pub enum RelayMessage {
    Event(EventSigned),
}

pub enum RelayStatus {
    Connected,
    Connecting,
    NotConnected,
}

pub struct Subscription {
    pub id: String,
    pub events: Vec<EventSigned>,
    pub is_finished: bool,
}

pub struct Relay {
    pub status: RelayStatus,
    pub socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>>,
    pub messages_memory: Arc<Mutex<Vec<RelayMessage>>>,
    pub subscriptions: Arc<Mutex<Vec<Arc<Mutex<Subscription>>>>>,
    pub url: String,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum ClientError {
    #[error("Error while connecting to the websocket server")]
    ConnectionError,

    #[error("Error parsing the websocket url, the url must be in the format wss://<host>:<port>")]
    UrlParseError,

    #[error("Error while sending the message to the websocket server")]
    SendMessageError,

    #[error("Error while receiving the message from the websocket server")]
    ReceiveMessageError,
}

impl Relay {
    pub async fn new(relay_url: &str) -> Result<Self, ClientError> {
        // MARK: - RELAY SOCKET
        let url = match Url::parse(relay_url) {
            Ok(url) => url,
            Err(_) => return Err(ClientError::UrlParseError),
        };

        let (socket, _) = match connect_async(&url).await {
            Ok((socket, response)) => (socket, response),
            Err(_) => return Err(ClientError::ConnectionError),
        };

        // MARK: - RELAY CONNECTION
        let relay = Relay {
            status: RelayStatus::Connected,
            socket: Arc::new(Mutex::new(socket)),
            messages_memory: Arc::new(Default::default()),
            subscriptions: Arc::new(Mutex::new(Vec::new())),
            url: relay_url.to_string(),
        };

        Ok(relay)
    }

    pub async fn read_message(&mut self) -> Result<Message, ClientError> {
        match self.socket.lock().await.next().await {
            Some(Ok(message)) => Ok(message),
            Some(Err(_)) => Err(ClientError::ReceiveMessageError),
            None => Err(ClientError::ReceiveMessageError),
        }
    }

    pub async fn listen(&mut self) {
        loop {
            let message = match self.read_message().await {
                Ok(message) => message,
                Err(_) => continue,
            };

            let message = match message.to_text() {
                Ok(message) => message,
                Err(_) => continue,
            };

            let message = match serde_json::from_str::<Value>(message) {
                Ok(message) => message,
                Err(_) => continue,
            };

            let message_array = match message.as_array() {
                Some(message_array) => message_array,
                None => continue,
            };

            let message_array_type = match message_array.get(0) {
                Some(message_array) => message_array,
                None => continue,
            };

            let message_array_type = match message_array_type.as_str() {
                Some(message_array_type) => message_array_type,
                None => continue,
            };

            match message_array_type {
                "event" => {
                    let event = match serde_json::from_value::<EventSigned>(message_array[1].clone()) {
                        Ok(event) => event,
                        Err(_) => continue,
                    };

                    self.messages_memory.lock().await.push(RelayMessage::Event(event.clone()));

                    for subscription in self.subscriptions.lock().await.iter_mut() {
                        if subscription.lock().await.id == event.id {
                            subscription.lock().await.events.push(event.clone());
                        }
                    }
                },
                _ => continue,
            }
        }
    }

    pub async fn send(
        &mut self,
        content: &str,
    ) -> Result<(), ClientError> {
        println!("Sending message: {content}");
        let message = crate::Message::text(content);

        match self.socket.lock().await.send(message.clone()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(ClientError::SendMessageError),
        }
    }

    pub async fn subscribe(
        &mut self,
        filters: Vec<ReqFilter>,
    ) -> Result<Arc<Mutex<Vec<Arc<Mutex<Subscription>>>>>, ClientError> {
        let req = Req::new(None, filters);
        self.send(&req.to_string()).await?;

        let subscription = Arc::new(Mutex::new(Subscription {
            id: req.subscription_id.clone(),
            events: Vec::new(),
            is_finished: false,
        }));

        self.subscriptions.lock().await.push();

        Ok(req.subscription_id)
    }
}

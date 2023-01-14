// Simplified websocket implementation
use futures::StreamExt;
use futures_util::sink::SinkExt;
use thiserror::Error;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum SimplifiedWSError {
    #[error("Error while connecting to the websocket server")]
    ConnectionError,

    #[error("Error parsing the websocket url, the url must be in the format wss://<host>:<port>")]
    UrlParseError,

    #[error("Error while sending the message to the websocket server")]
    SendMessageError,

    #[error("Error while receiving the message from the websocket server")]
    ReceiveMessageError,
}

pub struct SimplifiedWS {
    pub url: Url,
    pub socket: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
}

impl SimplifiedWS {
    pub async fn new(url: &str) -> Result<Self, SimplifiedWSError> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => return Err(SimplifiedWSError::UrlParseError),
        };

        let (socket, _) = match connect_async(&url).await {
            Ok((socket, response)) => (socket, response),
            Err(_) => return Err(SimplifiedWSError::ConnectionError),
        };

        Ok(Self { url, socket })
    }

    pub async fn send_message(&mut self, message: &Message) -> Result<(), SimplifiedWSError> {
        match self.socket.send(message.clone()).await {
            Ok(_) => Ok(()),
            Err(_) => Err(SimplifiedWSError::SendMessageError),
        }
    }

    pub async fn read_message(&mut self) -> Result<Message, SimplifiedWSError> {
        match self.socket.next().await {
            Some(Ok(message)) => Ok(message),
            Some(Err(_)) => Err(SimplifiedWSError::ReceiveMessageError),
            None => Err(SimplifiedWSError::ReceiveMessageError),
        }
    }
}

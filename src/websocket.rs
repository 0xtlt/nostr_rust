// Simplified websocket implementation
use std::net::TcpStream;
use thiserror::Error;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
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
    pub socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl SimplifiedWS {
    pub fn new(url: &str) -> Result<Self, SimplifiedWSError> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(_) => return Err(SimplifiedWSError::UrlParseError),
        };

        let (socket, _) = match connect(&url) {
            Ok((socket, response)) => (socket, response),
            Err(_) => return Err(SimplifiedWSError::ConnectionError),
        };

        Ok(Self { url, socket })
    }

    pub fn send_message(&mut self, message: &Message) -> Result<(), SimplifiedWSError> {
        match self.socket.write_message(message.clone()) {
            Ok(_) => Ok(()),
            Err(_) => Err(SimplifiedWSError::SendMessageError),
        }
    }

    pub fn read_message(&mut self) -> Result<Message, SimplifiedWSError> {
        match self.socket.read_message() {
            Ok(message) => Ok(message),
            Err(_) => Err(SimplifiedWSError::ReceiveMessageError),
        }
    }
}

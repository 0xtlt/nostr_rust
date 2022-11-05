// Simplified websocket implementation
use std::net::TcpStream;
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url::Url;

pub struct SimplifiedWS {
    pub url: Url,
    pub socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl SimplifiedWS {
    pub fn new(url: &str) -> Result<Self, String> {
        let url = match Url::parse(url) {
            Ok(url) => url,
            Err(err) => return Err(format!("Error parsing url: {}", err)),
        };

        let (socket, _) = match connect(&url) {
            Ok((socket, response)) => (socket, response),
            Err(err) => return Err(format!("Error connecting to websocket: {}", err)),
        };

        Ok(Self { url, socket })
    }

    pub fn send_message(&mut self, message: &Message) -> Result<(), String> {
        match self.socket.write_message(message.clone()) {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Error sending message: {}", err)),
        }
    }

    pub fn read_message(&mut self) -> Result<Message, String> {
        match self.socket.read_message() {
            Ok(message) => Ok(message),
            Err(err) => Err(format!("Error reading message: {}", err)),
        }
    }
}

use crate::{events::Event, nostr_client::Client, req::ReqFilter};
use std::collections::HashMap;

impl Client {
    pub fn listen(&self) {
        loop {
            let events = self.next_data().unwrap();

            for (relay_url, message) in events.iter() {
                match message {
                    tungstenite::Message::Text(_) => todo!(),
                    tungstenite::Message::Binary(_) => todo!(),
                    tungstenite::Message::Ping(_) => todo!(),
                    tungstenite::Message::Pong(_) => todo!(),
                    tungstenite::Message::Close(_) => todo!(),
                    tungstenite::Message::Frame(_) => todo!(),
                }
            }
        }
    }
}

pub struct SubscriptionPool {
    // will panic if is listening is false
    is_listening: bool,
    subscriptions: HashMap<String, (Vec<ReqFilter>, Vec<SubscriptionMessage>)>,
}

impl SubscriptionPool {
    pub fn new() -> Self {
        Self {
            is_listening: false,
            subscriptions: HashMap::new(),
        }
    }

    pub fn listen(&mut self) {
        self.is_listening = true;
    }

    pub fn stop(&mut self) {
        self.is_listening = false;
    }

    pub fn is_listening(&self) -> bool {
        self.is_listening
    }
}

impl Default for SubscriptionPool {
    fn default() -> Self {
        Self::new()
    }
}

pub enum SubscriptionMessage {
    Event(Event),
}

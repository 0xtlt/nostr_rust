use crate::events::Event;
use crate::req::{Req, ReqFilter};
use crate::websocket::SimplifiedWS;
use serde_json::json;
use tungstenite::Message;

/// Relay Type contains the relay address and the websocket connection
pub type Relay = (String, SimplifiedWS);

/// Nostr Client
pub struct Client {
    pub relays: Vec<Relay>,
}

impl Client {
    /// Create a new client with a list of default relays
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// let client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// ```
    pub fn new(default_relays: Vec<&str>) -> Result<Self, String> {
        let mut client = Self { relays: vec![] };

        for relay in default_relays {
            client.add_relay(relay)?;
        }

        Ok(client)
    }
}

impl Client {
    /// Add a relay to the client
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// client.add_relay("wss://nostr-pub.wellorder.net").unwrap();
    /// ```
    pub fn add_relay(&mut self, relay: &str) -> Result<(), String> {
        let client = match SimplifiedWS::new(relay) {
            Ok(client) => client,
            Err(err) => return Err(format!("Error connecting to relay: {}", err)),
        };

        self.relays.push((relay.to_string(), client));

        Ok(())
    }

    /// Publish a Nostr event
    pub fn publish_event(&mut self, event: &Event) -> Result<(), String> {
        let json_stringified = json!(["EVENT", event]).to_string();
        let message = Message::text(json_stringified);
        match self.relays[0].1.send_message(&message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to send message".to_string()),
        }
    }

    /// Listen for data from the relays
    /// # Example
    /// ```rust
    /// use std::{
    ///  sync::{Arc, Mutex},
    ///  thread,
    /// };
    /// use tungstenite::Message;
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    ///
    /// fn handle_message(relay_url: String, message: Message) -> Result<(), String> {
    ///   println!("Received message: {:?}", message);
    ///
    ///   Ok(())
    /// }
    ///
    /// let mut client = Arc::new(Mutex::new(Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap()));
    ///
    /// // Run a new thread to listen
    /// let nostr_clone = client.clone();
    /// let nostr_thread = thread::spawn(move || {
    ///     println!("Listening...");
    ///     nostr_clone.lock().unwrap().listen(handle_message).unwrap();
    /// });
    ///
    /// // Subscribe to the most beautiful Nostr profile event
    /// client
    /// .lock()
    /// .unwrap()
    /// .subscribe(vec![ReqFilter {
    ///     ids: None,
    ///     authors: Some(vec![
    ///         "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///     ]),
    ///     kinds: None,
    ///     e: None,
    ///     p: None,
    ///     since: None,
    ///     until: None,
    ///     limit: Some(1),
    /// }])
    /// .unwrap();
    ///
    /// // Wait 2s for the thread to finish
    /// std::thread::sleep(std::time::Duration::from_secs(2));
    /// ```
    pub fn listen<F, E>(&mut self, callback: F) -> Result<(), E>
    where
        F: Fn(String, Message) -> Result<(), E>,
    {
        for relay in &mut self.relays {
            let client = &mut relay.1;
            println!("Listening for messages from relay {}", relay.0);
            loop {
                match client.read_message() {
                    Ok(message) => callback(relay.0.clone(), message),
                    Err(err) => {
                        println!("Error reading message: {}", err);
                        continue;
                    }
                }?;
            }
        }

        Ok(())
    }

    /// Subscribe
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// client
    /// .subscribe(vec![ReqFilter { // None means generate a random ID
    ///     ids: None,
    ///     authors: Some(vec![
    ///         "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///     ]),
    ///     kinds: None,
    ///     e: None,
    ///     p: None,
    ///     since: None,
    ///     until: None,
    ///     limit: Some(1),
    /// }])
    /// .unwrap();
    /// ```
    pub fn subscribe(&mut self, filters: Vec<ReqFilter>) -> Result<String, String> {
        let req = Req::new(None, filters);
        let message = Message::text(req.to_string());
        match self.relays[0].1.send_message(&message) {
            Ok(_) => Ok(req.subscription_id),
            Err(_) => Err("Unable to send message".to_string()),
        }
    }

    /// Subscribe with a specific ID
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// client
    /// .subscribe_with_id("my_subscription_id", vec![ReqFilter {
    ///    ids: None,
    ///    authors: Some(vec![
    ///      "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///    ]),
    ///    kinds: None,
    ///    e: None,
    ///    p: None,
    ///    since: None,
    ///    until: None,
    ///    limit: Some(1),
    /// }])
    /// .unwrap();
    /// ```
    pub fn subscribe_with_id(
        &mut self,
        subscription_id: &str,
        filters: Vec<ReqFilter>,
    ) -> Result<(), String> {
        let req = Req::new(Some(subscription_id), filters);
        let message = Message::text(req.to_string());
        match self.relays[0].1.send_message(&message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to send message".to_string()),
        }
    }

    /// Unsubscribe
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let subscription_id = client
    /// .subscribe(vec![ReqFilter {
    ///    ids: None,
    ///   authors: Some(vec![
    ///        "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///   ]),
    ///  kinds: None,
    ///  e: None,
    ///  p: None,
    ///  since: None,
    ///  until: None,
    ///  limit: Some(1),
    /// }])
    /// .unwrap();
    /// client.unsubscribe(&subscription_id).unwrap();
    /// ```
    pub fn unsubscribe(&mut self, subscription_id: &str) -> Result<(), String> {
        let message = Message::text(json!(["CLOSE", subscription_id]).to_string());
        match self.relays[0].1.send_message(&message) {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to send message".to_string()),
        }
    }
}

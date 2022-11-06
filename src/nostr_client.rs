use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::events::Event;
use crate::req::{Req, ReqFilter};
use crate::websocket::SimplifiedWS;
use serde_json::json;
use tungstenite::Message;

/// Nostr Client
pub struct Client {
    pub relays: HashMap<String, Arc<Mutex<SimplifiedWS>>>,
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
        let mut client = Self {
            relays: HashMap::new(),
        };

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
    /// client.add_relay("wss://relay.damus.io").unwrap();
    /// ```
    pub fn add_relay(&mut self, relay: &str) -> Result<(), String> {
        let client = match SimplifiedWS::new(relay) {
            Ok(client) => client,
            Err(err) => return Err(format!("Error connecting to relay: {}", err)),
        };

        // Check if relay is already added
        if self.relays.contains_key(relay) {
            return Err(format!("Relay {} already added", relay));
        }

        self.relays
            .insert(relay.to_string(), Arc::new(Mutex::new(client)));

        Ok(())
    }

    /// Remove a relay from the client
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// client.remove_relay("wss://nostr-pub.wellorder.net").unwrap();
    /// ```
    pub fn remove_relay(&mut self, relay: &str) -> Result<(), String> {
        println!("Removing relay {}", relay);
        if !self.relays.contains_key(relay) {
            println!("Relay {} not found", relay);
            return Err(format!("Relay {} not found", relay));
        }

        println!("Removing relay {}", relay);

        // Close the connection
        self.relays
            .remove(relay)
            .unwrap()
            .lock()
            .unwrap()
            .socket
            .close(None)
            .unwrap();

        Ok(())
    }

    /// Publish a Nostr event
    pub fn publish_event(&mut self, event: &Event) -> Result<(), String> {
        let json_stringified = json!(["EVENT", event]).to_string();
        let message = Message::text(json_stringified);

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
    }

    /// Get next data from the relays
    /// # Example
    /// ```rust
    /// use std::{
    ///  sync::{Arc, Mutex},
    ///  thread,
    /// };
    /// use tungstenite::Message;
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    ///
    /// fn handle_message(relay_url: &String, message: &Message) -> Result<(), String> {
    ///   println!("Received message: {:?}", message);
    ///
    ///   Ok(())
    /// }
    ///
    /// let mut client = Arc::new(Mutex::new(Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap()));
    ///
    /// // Run a new thread to listen
    /// let nostr_clone = client.clone();
    /// let nostr_thread = thread::spawn(move || loop {
    ///    let events = nostr_clone.lock().unwrap().next_data().unwrap();
    ///    
    ///   for (relay_url, message) in events.iter() {
    ///     handle_message(relay_url, message).unwrap();
    ///   }
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
    /// // Wait 3s for the thread to finish
    /// std::thread::sleep(std::time::Duration::from_secs(3));
    /// ```
    pub fn next_data(&mut self) -> Result<Vec<(String, tungstenite::Message)>, String> {
        let mut events: Vec<(String, tungstenite::Message)> = Vec::new();

        for (relay_name, socket) in self.relays.iter() {
            let message = socket.lock().unwrap().read_message()?;
            events.push((relay_name.clone(), message));
        }

        Ok(events)
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

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(req.subscription_id)
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

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
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

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
    }
}

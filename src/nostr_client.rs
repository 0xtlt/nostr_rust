use crate::events::Event;
use crate::req::{Req, ReqFilter};
use crate::websocket::{self, SimplifiedWS};
use crate::Message;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("Error while trying to connect to the websocket server")]
    WSError(websocket::SimplifiedWSError),

    #[error("Already subscribed to the event")]
    AlreadySubscribed,

    #[error("Relay does not exist")]
    RelayDoesNotExist,

    #[error("Serde Error: {}", _0)]
    SerdeError(#[from] serde_json::Error),
}

impl From<websocket::SimplifiedWSError> for ClientError {
    fn from(err: websocket::SimplifiedWSError) -> Self {
        Self::WSError(err)
    }
}

#[cfg(not(feature = "async"))]
/// Nostr Client
pub struct Client {
    pub relays: HashMap<String, Arc<std::sync::Mutex<SimplifiedWS>>>,
    pub subscriptions: HashMap<String, Vec<Message>>,
}

#[cfg(feature = "async")]
/// Nostr Client
pub struct Client {
    pub relays: HashMap<String, Arc<tokio::sync::Mutex<SimplifiedWS>>>,
    pub subscriptions: HashMap<String, Vec<Message>>,
}

impl Client {
    #[cfg(not(feature = "async"))]
    /// Create a new client with a list of default relays
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// let client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// ```
    pub fn new(default_relays: Vec<&str>) -> Result<Self, ClientError> {
        let mut client = Self {
            relays: HashMap::new(),
            subscriptions: HashMap::new(),
        };

        for relay in default_relays {
            client.add_relay(relay)?;
        }

        Ok(client)
    }

    #[cfg(feature = "async")]
    /// Create a new client with a list of default relays
    ///
    /// # Example
    /// ```rust, async_await
    /// use nostr_rust::nostr_client::Client;
    ///
    /// #[tokio::test]
    /// async fn test_new_client() {
    ///     let client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    /// }
    /// ```
    pub async fn new(default_relays: Vec<&str>) -> Result<Self, ClientError> {
        let mut client = Self {
            relays: HashMap::new(),
            subscriptions: HashMap::new(),
        };

        for relay in default_relays {
            client.add_relay(relay).await?;
        }

        Ok(client)
    }
}

impl Client {
    #[cfg(not(feature = "async"))]
    /// Add a relay to the client
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// let mut client = Client::new(vec![]).unwrap();
    /// client.add_relay(env!("RELAY_URL")).unwrap();
    /// ```
    pub fn add_relay(&mut self, relay: &str) -> Result<(), ClientError> {
        let client = match SimplifiedWS::new(relay) {
            Ok(client) => client,
            Err(err) => return Err(ClientError::WSError(err)),
        };

        // Check if relay is already added
        if self.relays.contains_key(relay) {
            return Err(ClientError::AlreadySubscribed);
        }

        self.relays
            .insert(relay.to_string(), Arc::new(std::sync::Mutex::new(client)));

        Ok(())
    }

    #[cfg(feature = "async")]
    /// Add a relay to the client
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    ///
    /// #[tokio::test]
    /// async fn test_add_relay() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     client.add_relay("wss://relay.damus.io").await.unwrap();
    /// }
    /// ```
    pub async fn add_relay(&mut self, relay: &str) -> Result<(), ClientError> {
        let client = match SimplifiedWS::new(relay).await {
            Ok(client) => client,
            Err(err) => return Err(ClientError::WSError(err)),
        };

        // Check if relay is already added
        if self.relays.contains_key(relay) {
            return Err(ClientError::AlreadySubscribed);
        }

        self.relays
            .insert(relay.to_string(), Arc::new(tokio::sync::Mutex::new(client)));

        Ok(())
    }

    #[cfg(not(feature = "async"))]
    /// Remove a relay from the client
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// client.remove_relay(env!("RELAY_URL")).unwrap();
    /// ```
    pub fn remove_relay(&mut self, relay: &str) -> Result<(), ClientError> {
        if !self.relays.contains_key(relay) {
            return Err(ClientError::RelayDoesNotExist);
        }

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

    #[cfg(feature = "async")]
    /// Remove a relay from the client
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    ///
    /// #[tokio::test]
    /// async fn test_remove_relay() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     client.remove_relay(env!("RELAY_URL")).await.unwrap();
    /// }
    /// ```
    pub async fn remove_relay(&mut self, relay: &str) -> Result<(), ClientError> {
        if !self.relays.contains_key(relay) {
            return Err(ClientError::RelayDoesNotExist);
        }

        // Close the connection
        self.relays
            .remove(relay)
            .unwrap()
            .lock()
            .await
            .socket
            .close(None)
            .await
            .unwrap();

        Ok(())
    }

    #[cfg(not(feature = "async"))]
    /// Publish a Nostr event
    pub fn publish_event(&mut self, event: &Event) -> Result<(), ClientError> {
        let json_stringified = json!(["EVENT", event]).to_string();
        let message = Message::text(json_stringified);

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    /// Publish a Nostr event
    pub async fn publish_event(&mut self, event: &Event) -> Result<(), ClientError> {
        let json_stringified = json!(["EVENT", event]).to_string();
        let message = Message::text(json_stringified);

        for relay in self.relays.values() {
            let mut relay = relay.lock().await;
            relay.send_message(&message).await?;
        }

        Ok(())
    }

    #[cfg(not(feature = "async"))]
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
    /// let mut client = Arc::new(Mutex::new(Client::new(vec![env!("RELAY_URL")]).unwrap()));
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
    pub fn next_data(&mut self) -> Result<Vec<(String, tungstenite::Message)>, ClientError> {
        let mut events: Vec<(String, tungstenite::Message)> = Vec::new();

        for (relay_name, socket) in self.relays.iter() {
            let message = socket.lock().unwrap().read_message()?;
            events.push((relay_name.clone(), message));
        }

        Ok(events)
    }

    #[cfg(feature = "async")]
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
    /// #[tokio::test]
    /// async fn test_next_data() {
    ///     let mut client = Arc::new(Mutex::new(Client::new(vec![env!("RELAY_URL")]).await.unwrap()));
    ///
    ///     // Run a new thread to listen
    ///     let nostr_clone = client.clone();
    ///     let nostr_thread = thread::spawn(move || loop {
    ///         let events = nostr_clone.lock().unwrap().next_data().await.unwrap();
    ///    
    ///         for (relay_url, message) in events.iter() {
    ///             handle_message(relay_url, message).unwrap();
    ///         }
    ///     });
    ///
    ///     // Subscribe to the most beautiful Nostr profile event
    ///     client
    ///     .lock()
    ///     .unwrap()
    ///     .subscribe(vec![ReqFilter {
    ///         ids: None,
    ///         authors: Some(vec![
    ///             "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///         ]),
    ///         kinds: None,
    ///         e: None,
    ///         p: None,
    ///         since: None,
    ///         until: None,
    ///         limit: Some(1),
    ///     }])
    ///     .await
    ///     .unwrap();
    ///
    /// // Wait 3s for the thread to finish
    /// std::thread::sleep(std::time::Duration::from_secs(3));
    /// }
    /// ```
    pub async fn next_data(&mut self) -> Result<Vec<(String, tungstenite::Message)>, ClientError> {
        let mut events: Vec<(String, tungstenite::Message)> = Vec::new();

        for (relay_name, socket) in self.relays.iter() {
            let message = socket.lock().await.read_message().await?;
            events.push((relay_name.clone(), message));
        }

        Ok(events)
    }

    #[cfg(not(feature = "async"))]
    /// Subscribe
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
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
    pub fn subscribe(&mut self, filters: Vec<ReqFilter>) -> Result<String, ClientError> {
        let req = Req::new(None, filters);
        let message = Message::text(req.to_string());

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(req.subscription_id)
    }

    #[cfg(feature = "async")]
    /// Subscribe
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    ///
    /// #[tokio::test]
    /// async fn test_subscribe() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     client
    ///     .subscribe(vec![ReqFilter { // None means generate a random ID
    ///         ids: None,
    ///         authors: Some(vec![
    ///             "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///         ]),
    ///         kinds: None,
    ///         e: None,
    ///         p: None,
    ///         since: None,
    ///         until: None,
    ///         limit: Some(1),
    ///     }])
    ///     .await
    ///     .unwrap();
    /// }
    /// ```
    pub async fn subscribe(&mut self, filters: Vec<ReqFilter>) -> Result<String, ClientError> {
        let req = Req::new(None, filters);
        let message = Message::text(req.to_string());

        for relay in self.relays.values() {
            let mut relay = relay.lock().await;
            relay.send_message(&message).await?;
        }

        Ok(req.subscription_id)
    }

    #[cfg(not(feature = "async"))]
    /// Subscribe with a specific ID
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
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
    ) -> Result<(), ClientError> {
        let req = Req::new(Some(subscription_id), filters);
        let message = Message::text(req.to_string());

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    /// Subscribe with a specific ID
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    ///
    /// #[tokio::test]
    /// async fn test_subscribe_with_id() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     client
    ///     .subscribe_with_id("my_subscription_id", vec![ReqFilter {
    ///        ids: None,
    ///        authors: Some(vec![
    ///          "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///        ]),
    ///        kinds: None,
    ///        e: None,
    ///        p: None,
    ///        since: None,
    ///        until: None,
    ///        limit: Some(1),
    ///     }])
    ///     .await
    ///     .unwrap();
    /// }
    /// ```
    pub async fn subscribe_with_id(
        &mut self,
        subscription_id: &str,
        filters: Vec<ReqFilter>,
    ) -> Result<(), ClientError> {
        let req = Req::new(Some(subscription_id), filters);
        let message = Message::text(req.to_string());

        for relay in self.relays.values() {
            let mut relay = relay.lock().await;
            relay.send_message(&message).await?;
        }

        Ok(())
    }

    #[cfg(not(feature = "async"))]
    /// Unsubscribe
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
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
    pub fn unsubscribe(&mut self, subscription_id: &str) -> Result<(), ClientError> {
        let message = Message::text(json!(["CLOSE", subscription_id]).to_string());

        for relay in self.relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    /// Unsubscribe
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    ///
    /// #[tokio::test]
    /// async fn test_unsubscribe() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let subscription_id = client
    ///     .subscribe(vec![ReqFilter {
    ///        ids: None,
    ///       authors: Some(vec![
    ///            "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///       ]),
    ///      kinds: None,
    ///      e: None,
    ///      p: None,
    ///      since: None,
    ///      until: None,
    ///      limit: Some(1),
    ///     }])
    ///     .await
    ///     .unwrap();
    ///     client.unsubscribe(&subscription_id).await.unwrap();
    /// }
    /// ```
    pub async fn unsubscribe(&mut self, subscription_id: &str) -> Result<(), ClientError> {
        let message = Message::text(json!(["CLOSE", subscription_id]).to_string());

        for relay in self.relays.values() {
            let mut relay = relay.lock().await;
            relay.send_message(&message).await?;
        }

        Ok(())
    }

    /// Add event to a subscription
    pub fn add_event(&mut self, subscription_id: &str, message: Message) {
        // Check if the subscription exists
        if !self.subscriptions.contains_key(subscription_id) {
            self.subscriptions
                .insert(subscription_id.to_string(), Vec::new());
        }

        // Check if the message is already in the subscription
        if !self.subscriptions[subscription_id].contains(&message) {
            // Add the message to the subscription
            self.subscriptions
                .get_mut(subscription_id)
                .unwrap()
                .push(message);
        }
    }

    /// Get events and remove them from the subscription
    pub fn get_events(&mut self, subscription_id: &str) -> Option<Vec<Message>> {
        self.subscriptions.remove(subscription_id)
    }

    #[cfg(not(feature = "async"))]
    /// Get events of a given filters
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let events = client.get_events_of(vec![ReqFilter {
    ///    ids: None,
    ///    authors: Some(vec!["884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string()]),
    ///    kinds: Some(vec![3]),
    ///    e: None,
    ///    p: None,
    ///    since: None,
    ///    until: None,
    ///    limit: Some(1),
    /// }]).unwrap();
    /// ```
    pub fn get_events_of(&mut self, filters: Vec<ReqFilter>) -> Result<Vec<Event>, ClientError> {
        let mut events: Vec<Event> = Vec::new();

        // Subscribe
        let id = self.subscribe(filters)?;

        let mut waiting_relays: Vec<String> = self.relays.keys().map(|k| k.to_string()).collect();

        // Get the events
        while !waiting_relays.is_empty() {
            let data = self.next_data()?;
            let mut break_loop = false;

            for (relay, message) in data {
                let event: Value = serde_json::from_str(&message.to_string())?;

                if event[0] == "EOSE" && event[1].as_str() == Some(&id) {
                    let index = waiting_relays.iter().position(|r| r == &relay).unwrap();
                    waiting_relays.remove(index);

                    break_loop = true;
                    break;
                }

                self.add_event(&id, message);
            }

            if break_loop {
                break;
            }
        }

        // unsubscribe
        self.unsubscribe(&id)?;

        // Get the events
        if let Some(messages) = self.get_events(&id) {
            for message in messages {
                if !message.is_text() {
                    continue;
                }

                let event: Value = serde_json::from_str(&message.to_string())?;

                let event_object = serde_json::from_value::<Event>(event[2].clone());

                if event_object.is_err() {
                    continue;
                }

                events.push(event_object.unwrap());
            }
        }
        Ok(events)
    }

    #[cfg(feature = "async")]
    /// Get events of a given filters
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, req::ReqFilter};
    ///
    /// #[tokio::test]
    /// async fn test_get_events_of() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let events = client.get_events_of(vec![ReqFilter {
    ///        ids: None,
    ///        authors: Some(vec!["884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string()]),
    ///        kinds: Some(vec![3]),
    ///        e: None,
    ///        p: None,
    ///        since: None,
    ///        until: None,
    ///        limit: Some(1),
    ///     }]).await
    ///     .unwrap();
    /// }
    /// ```
    pub async fn get_events_of(
        &mut self,
        filters: Vec<ReqFilter>,
    ) -> Result<Vec<Event>, ClientError> {
        let mut events: Vec<Event> = Vec::new();

        // Subscribe
        let id = self.subscribe(filters).await?;

        let mut waiting_relays: Vec<String> = self.relays.keys().map(|k| k.to_string()).collect();

        // Get the events
        while !waiting_relays.is_empty() {
            let data = self.next_data().await?;
            let mut break_loop = false;

            for (relay, message) in data {
                let event: Value = serde_json::from_str(&message.to_string()).unwrap();

                if event[0] == "EOSE" && event[1].as_str() == Some(&id) {
                    let index = waiting_relays.iter().position(|r| r == &relay).unwrap();
                    waiting_relays.remove(index);

                    break_loop = true;
                    break;
                }

                self.add_event(&id, message);
            }

            if break_loop {
                break;
            }
        }

        // unsubscribe
        self.unsubscribe(&id).await?;

        // Get the events
        if let Some(messages) = self.get_events(&id) {
            for message in messages {
                if !message.is_text() {
                    continue;
                }

                let event: Value = serde_json::from_str(&message.to_string())?;

                let event_object = serde_json::from_value::<Event>(event[2].clone());

                if event_object.is_err() {
                    continue;
                }

                events.push(event_object.unwrap());
            }
        }
        Ok(events)
    }
}

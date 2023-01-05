use crate::websocket::{self, SimplifiedWS};
use std::sync::Arc;

use crate::{
    events::{Event, EventPrepare},
    nips::nip11,
    nostr_client::{Client, ClientError},
    utils::get_timestamp,
    Identity, Message,
};
use serde_json::json;
use std::collections::HashMap;
use thiserror::Error;

// Implementation of the NIP16 protocol
// https://github.com/nostr-protocol/nips/blob/master/16.md

#[derive(Error, Debug)]
pub enum NIP16Error {
    #[error("Error while trying to connect to the websocket server")]
    WSError(websocket::SimplifiedWSError),

    #[error("Event Kind outside range")]
    EventKindOutOfRange,

    #[error("The client has an error")]
    ClientError(ClientError),
}

impl From<ClientError> for NIP16Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

impl From<websocket::SimplifiedWSError> for NIP16Error {
    fn from(err: websocket::SimplifiedWSError) -> Self {
        Self::WSError(err)
    }
}

impl Client {
    #[cfg(not(feature = "async"))]
    /// Publish a replaceable event.
    /// `kind` argument should be less then 9999.
    /// `publish_replaceable_event` adds 10000 to `kind` to update event `kind` to be within the NIP16
    /// replaceable event range 10000 <= kind < 20000
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// use nostr_rust::Identity;
    /// use nostr_rust::nips::nip16::NIP16Error;
    /// use std::str::FromStr;

    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let event = client.publish_replaceable_event(
    ///  &identity,
    ///  20000,
    ///  "hello world",
    ///  &[],
    ///  0).unwrap_err();
    /// assert_eq!(format!("{:?}", event), "EventKindOutOfRange");
    ///
    /// let event = client.publish_replaceable_event(
    ///  &identity,
    ///  10,
    ///  "hello world",
    ///  &[],
    ///  0).unwrap();
    /// assert_eq!(event.kind, 10010)
    /// ```
    pub fn publish_replaceable_event(
        &mut self,
        identity: &Identity,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP16Error> {
        if kind > 9999 {
            return Err(NIP16Error::EventKindOutOfRange);
        }

        let kind = kind + 10000;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_nip16_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Publish a replaceable event.
    /// `kind` argument should be less then 9999.
    /// `publish_replaceable_event` adds 10000 to `kind` to update event `kind` to be within the NIP16
    /// replaceable event range 10000 <= kind < 20000
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// use nostr_rust::Identity;
    /// use nostr_rust::nips::nip16::NIP16Error;
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_publish_replaceable_event() {
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let event = client.publish_replaceable_event(
    ///      &identity,
    ///       20000,
    ///      "hello world",
    ///       &[],
    ///       0).await.unwrap_err();
    ///     assert_eq!(event, NIP16Error::EventKindOutOfRange);
    ///
    ///     let event = client.publish_replaceable_event(
    ///      &identity,
    ///      10,
    ///      "hello world",
    ///      &[],
    ///      0).await.unwrap();
    ///     assert_eq!(event.kind, 10010)
    /// }
    /// ```
    pub async fn publish_replaceable_event(
        &mut self,
        identity: &Identity,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP16Error> {
        if kind > 9999 {
            return Err(NIP16Error::EventKindOutOfRange);
        }

        let kind = kind + 10000;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_nip16_event(&event).await?;
        Ok(event)
    }

    #[cfg(not(feature = "async"))]
    /// Publish an ephemeral event.
    /// `kind` argument should be less then 9999.
    /// `publish_ephemeral_event` adds 20000 to `kind` to update event `kind` to be within the NIP16
    /// ephemeral event range of 20000 <= kind < 30000
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// use nostr_rust::Identity;
    /// use nostr_rust::nips::nip16::NIP16Error;
    /// use std::str::FromStr;

    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let event = client.publish_ephemeral_event(
    ///                         &identity,
    ///                         10000,
    ///                         "hello world",
    ///                         &[],
    ///                         0)
    ///                         .unwrap_err();
    /// assert_eq!(format!("{:?}",event), format!("{:?}",NIP16Error::EventKindOutOfRange));
    /// let event = client.publish_ephemeral_event(&identity, 5, "hello world", &[],
    /// 0).unwrap();
    /// assert_eq!(event.kind, 20005);
    ///
    /// ```
    pub fn publish_ephemeral_event(
        &mut self,
        identity: &Identity,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP16Error> {
        if kind > 9999 {
            return Err(NIP16Error::EventKindOutOfRange);
        }

        let kind = kind + 20000;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_nip16_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Publish an ephemeral event.
    /// `kind` argument should be less then 9999.
    /// `publish_ephemeral_event` adds 20000 to `kind` to update event `kind` to be within the NIP16
    /// ephemeral event range of 20000 <= kind < 30000
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// use nostr_rust::Identity;
    /// use nostr_rust::nips::nip16::NIP16Error;
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_publish_ephemeral_event() {
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    ///     let event = client.publish_ephemeral_event_async(
    ///         &identity,
    ///         10000,
    ///         "hello world",
    ///         &[],
    ///         0).await.unwrap_err();
    ///     assert_eq!(event, NIP16Error::EventKindOutOfRange);
    ///     let event = client.publish_ephemeral_event_async(
    ///         &identity,
    ///         5,
    ///         "hello world",
    ///         &[],
    ///         0).await.unwrap();
    ///     assert_eq!(event.kind, 20005);
    /// }
    /// ```
    pub async fn publish_ephemeral_event_async(
        &mut self,
        identity: &Identity,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP16Error> {
        if kind > 9999 {
            return Err(NIP16Error::EventKindOutOfRange);
        }

        let kind = kind + 20000;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_nip16_event(&event).await?;
        Ok(event)
    }

    #[cfg(not(feature = "async"))]
    pub fn publish_nip16_event(&mut self, event: &Event) -> Result<(), NIP16Error> {
        let supported_relays: HashMap<&String, &Arc<std::sync::Mutex<SimplifiedWS>>> = self
            .relays
            .iter()
            .filter_map(|(relay_url, ws)| {
                if let Ok(relay_info) = nip11::get_relay_information_document(relay_url) {
                    if let Some(supported_nips) = relay_info.supported_nips {
                        if supported_nips.contains(&16) {
                            return Some((relay_url, ws));
                        }
                    }
                }
                None
            })
            .collect();

        let json_stringified = json!(["EVENT", event]).to_string();
        let message = Message::text(json_stringified);

        for relay in supported_relays.values() {
            let mut relay = relay.lock().unwrap();
            relay.send_message(&message)?;
        }

        Ok(())
    }

    #[cfg(feature = "async")]
    pub async fn publish_nip16_event(&mut self, event: &Event) -> Result<(), NIP16Error> {
        let mut supported_relays: HashMap<&String, &Arc<tokio::sync::Mutex<SimplifiedWS>>> =
            HashMap::new();

        for relay in self.relays.iter() {
            if let Ok(relay_info) = nip11::get_relay_information_document(relay.0).await {
                if let Some(supported_nips) = relay_info.supported_nips {
                    if supported_nips.contains(&16) {
                        supported_relays.insert(relay.0, relay.1);
                    }
                }
            }
        }

        let json_stringified = json!(["EVENT", event]).to_string();
        let message = Message::text(json_stringified);

        for relay in supported_relays.values() {
            let mut relay = relay.lock().await;
            relay.send_message(&message).await?;
        }

        Ok(())
    }
}

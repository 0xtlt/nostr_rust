use crate::websocket::{self, SimplifiedWS};
use std::sync::{Arc, Mutex};

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

#[derive(Error, Debug, Eq, PartialEq)]
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
    /// Publish a replaceable event
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// use nostr_rust::Identity;
    /// use nostr_rust::nips::nip16::NIP16Error;
    /// use std::str::FromStr;

    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let event = client.publish_replaceable_event(&identity, 20000, "hello world", &[],
    /// 0).unwrap_err();
    /// //assert_eq!(event, NIP16Error::EventKindOutOfRange);
    /// let event = client.publish_replaceable_event(&identity, 100, "hello world", &[],
    /// 0).unwrap_err();
    /// //assert_eq!(event, NIP16Error::EventKindOutOfRange);
    ///
    /// let event = client.publish_replaceable_event(&identity, 15000, "hello world", &[],
    /// 0).unwrap();
    /// ```
    pub fn publish_replaceable_event(
        &mut self,
        identity: &Identity,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP16Error> {
        if !(10000..20000).contains(&kind) {
            return Err(NIP16Error::EventKindOutOfRange);
        }

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

    /// Publish an ephemeral event
    /// # Example
    /// ```rust
    /// use nostr_rust::nostr_client::Client;
    /// use nostr_rust::Identity;
    /// use nostr_rust::nips::nip16::NIP16Error;
    /// use std::str::FromStr;

    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let event = client.publish_ephemeral_event(&identity, 19999, "hello world", &[],
    /// 0).unwrap_err();
    /// //assert_eq!(event, NIP16Error::EventKindOutOfRange);
    /// let event = client.publish_ephemeral_event(&identity, 30000, "hello world", &[],
    /// 0).unwrap_err();
    /// //assert_eq!(event, NIP16Error::EventKindOutOfRange);
    ///
    /// let event = client.publish_replaceable_event(&identity, 25000, "hello world", &[],
    /// 0).unwrap();
    /// ```
    pub fn publish_ephemeral_event(
        &mut self,
        identity: &Identity,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP16Error> {
        if !(20000..30000).contains(&kind) {
            return Err(NIP16Error::EventKindOutOfRange);
        }

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

    pub fn publish_nip16_event(&mut self, event: &Event) -> Result<(), NIP16Error> {
        let supported_relays: HashMap<&String, &Arc<Mutex<SimplifiedWS>>> = self
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
}

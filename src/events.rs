use std::fmt;

use secp256k1::{schnorr::Signature, KeyPair, XOnlyPublicKey, SECP256K1};
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::str::FromStr;

use thiserror::Error;

use crate::Identity;

/// EventPrepare is the struct used to prepare an event before publishing it (signing it and assigning it an id)
#[derive(Serialize, Deserialize, Debug)]
pub struct EventPrepare {
    /// 32-bytes hex-encoded public key of the event creator
    #[serde(rename = "pubkey")]
    pub pub_key: String,
    /// unix timestamp in seconds
    pub created_at: u64,
    /// integer
    /// 0: NostrEvent
    pub kind: u16,
    /// Tags
    pub tags: Vec<Vec<String>>,
    /// arbitrary string
    pub content: String,
}

impl EventPrepare {
    /// get_content returns the content of the event to be signed
    /// # Example
    /// ```rust
    /// use nostr_rust::{events::EventPrepare, utils::get_timestamp};
    ///
    /// let actual_time = get_timestamp();
    ///
    /// let event = EventPrepare {
    ///    pub_key: env!("PUBLIC_KEY").to_string(),
    ///    created_at: get_timestamp(),
    ///    kind: 0,
    ///    tags: vec![],
    ///    content: "content".to_string(),
    /// };
    ///
    /// assert_eq!(event.get_content(), format!("[0,\"c5aec31e83bdf980939b5ef7c6bcaa2be8bd39d38667da58ba6dba240eb8b69d\",{},0,[],\"content\"]", actual_time));
    /// ```
    pub fn get_content(&self) -> String {
        json!([
            0,
            self.pub_key,
            self.created_at,
            self.kind,
            self.tags,
            self.content
        ])
        .to_string()
    }

    /// Get the id of the event which is the sha256 hash of the content
    /// # Example
    /// ```rust
    /// use nostr_rust::{events::EventPrepare};
    ///
    /// let event = EventPrepare {
    ///   pub_key: env!("PUBLIC_KEY").to_string(),
    ///   created_at: 0, // Don't use this in production
    ///   kind: 0,
    ///   tags: vec![],
    ///   content: "content".to_string(),
    /// };
    ///
    /// assert_eq!(event.get_content_id(), "4a57aad22fc0fd374e8ceeaaaf8817fa6cb661ca2229c66309d7dba69dfe2359");
    /// ```
    pub fn get_content_id(&self) -> String {
        sha256::digest(self.get_content())
    }

    /// Transform the event to NostrEvent
    /// # Example
    /// ```rust
    /// use std::str::FromStr;
    /// use nostr_rust::{events::EventPrepare, Identity};
    ///
    /// let mut event = EventPrepare {
    ///  pub_key: env!("PUBLIC_KEY").to_string(),
    ///  created_at: 0, // Don't use this in production
    ///  kind: 0,
    ///  tags: vec![],
    ///  content: "content".to_string(),
    /// };
    ///
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// // Test to_event without Proof of Work
    /// let nostr_event = event.to_event(&identity, 0);
    /// assert_eq!(nostr_event.id, "4a57aad22fc0fd374e8ceeaaaf8817fa6cb661ca2229c66309d7dba69dfe2359");
    /// assert_eq!(nostr_event.content, "content");
    /// assert_eq!(nostr_event.kind, 0);
    /// assert_eq!(nostr_event.tags.len(), 0);
    /// assert_eq!(nostr_event.created_at, 0);
    /// assert_eq!(nostr_event.pub_key, env!("PUBLIC_KEY"));
    /// assert_eq!(nostr_event.sig.len(), 128);
    ///
    /// // Test to_event with Proof of Work
    /// let difficulty = 10;
    /// let mut nostr_event_pow = event.to_event(&identity, difficulty);
    /// let event_id = hex::decode(nostr_event_pow.id).unwrap();
    /// let event_difficulty = EventPrepare::count_leading_zero_bits(event_id);
    /// assert!(event_difficulty >= difficulty.into());
    /// assert_eq!(nostr_event_pow.content, "content");
    /// assert_eq!(nostr_event_pow.kind, 0);
    /// assert_eq!(nostr_event_pow.tags.len(), 1);
    /// assert!(nostr_event_pow.created_at > 0);
    /// assert_eq!(nostr_event_pow.pub_key, env!("PUBLIC_KEY"));
    /// assert_eq!(nostr_event_pow.sig.len(), 128);
    /// ```
    pub fn to_event(&mut self, secret_key: &Identity, difficulty_target: u16) -> Event {
        if difficulty_target > 0 {
            self.to_pow_event(difficulty_target).unwrap();
        }

        let message = secp256k1::Message::from_hashed_data::<secp256k1::hashes::sha256::Hash>(
            self.get_content().as_bytes(),
        );

        let signature = SECP256K1
            .sign_schnorr(
                &message,
                &KeyPair::from_secret_key(SECP256K1, &secret_key.secret_key),
            )
            .to_string();

        Event {
            id: self.get_content_id(),
            pub_key: self.pub_key.clone(),
            created_at: self.created_at,
            kind: self.kind,
            tags: self.tags.clone(),
            content: self.content.clone(),
            sig: signature,
        }
    }
}

/// Event is the struct used to represent a Nostr event
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    /// 32-bytes sha256 of the serialized event data
    pub id: String,
    /// 32-bytes hex-encoded public key of the event creator
    #[serde(rename = "pubkey")]
    pub pub_key: String,
    /// unix timestamp in seconds
    pub created_at: u64,
    /// integer
    /// 0: NostrEvent
    pub kind: u16,
    /// Tags
    pub tags: Vec<Vec<String>>,
    /// arbitrary string
    pub content: String,
    /// 64-bytes signature of the sha256 hash of the serialized event data, which is the same as the "id" field
    pub sig: String,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum EventError {
    #[error("Secp256k1 Error: {}", _0)]
    Secp256k1Error(secp256k1::Error),
}

impl From<secp256k1::Error> for EventError {
    fn from(err: secp256k1::Error) -> Self {
        Self::Secp256k1Error(err)
    }
}

impl Event {
    /// get_content returns the content of the event
    /// # Example
    /// ```rust
    /// use nostr_rust::{events::EventPrepare, utils::get_timestamp, Identity};
    /// use std::str::FromStr;
    ///
    /// let actual_time = get_timestamp();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let event = EventPrepare {
    ///    pub_key: env!("PUBLIC_KEY").to_string(),
    ///    created_at: get_timestamp(),
    ///    kind: 0,
    ///    tags: vec![],
    ///    content: "content".to_string(),
    /// }.to_event(&identity, 0);
    /// assert_eq!(event.get_content(), format!("[0,\"{}\",{},0,[],\"content\"]", env!("PUBLIC_KEY"), actual_time));
    /// ```
    pub fn get_content(&self) -> String {
        json!([
            0,
            self.pub_key,
            self.created_at,
            self.kind,
            self.tags,
            self.content
        ])
        .to_string()
    }

    /// Get the id of the event which is the sha256 hash of the content
    /// # Example
    /// ```rust
    /// use nostr_rust::{events::EventPrepare, Identity};
    /// use std::str::FromStr;
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let event = EventPrepare {
    ///   pub_key: env!("PUBLIC_KEY").to_string(),
    ///   created_at: 0, // Don't use this in production
    ///   kind: 0,
    ///   tags: vec![],
    ///   content: "content".to_string(),
    /// }.to_event(&identity, 0);
    ///
    /// assert_eq!(event.get_content_id().len(), 64);
    /// ```
    pub fn get_content_id(&self) -> String {
        sha256::digest(self.get_content())
    }

    /// Get the id of the event which is the sha256 hash of the content
    /// # Example
    /// ```rust
    /// use nostr_rust::{events::EventPrepare, Identity};
    /// use std::str::FromStr;
    ///
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// let event = EventPrepare {
    ///   pub_key: env!("PUBLIC_KEY").to_string(),
    ///   created_at: 0, // Don't use this in production
    ///   kind: 0,
    ///   tags: vec![],
    ///   content: "content".to_string(),
    /// }.to_event(&identity, 0);
    ///
    /// event.verify().unwrap()
    /// ```
    pub fn verify(&self) -> Result<(), EventError> {
        let message = secp256k1::Message::from_hashed_data::<secp256k1::hashes::sha256::Hash>(
            self.get_content().as_bytes(),
        );

        SECP256K1.verify_schnorr(
            &Signature::from_str(&self.sig)?,
            &message,
            &XOnlyPublicKey::from_str(&self.pub_key)?,
        )?;
        Ok(())
    }
}

impl fmt::Display for Event {
    /// Return the serialized event
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

/// Extract events from a string
///
/// # Example
/// ```rust
/// use nostr_rust::events::extract_events;
///
/// let txt = "[\"EVENT\",\"deb0ab5bd829d1642c926b7897b078d027ca41870d0a499c1fd76e4b5af5ccbd\",{\"id\":\"f0382d932ddc5876bad3f9c5fdb84fb4c2af7ccefebfb491f13fbc47c38f8ae4\",\"kind\":1,\"pubkey\":\"884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6\",\"created_at\":1673131597,\"content\":\"Does anyone know a good crate rust to handle a Lightning node?\",\"tags\":[],\"sig\":\"53a629bae11dace9b487700cbe8e85058a3d7b6989e1e0bdd6eb4fb0201a3779742682f65ca37782c0cb93019a170e0a368bb033dfce1102df71420e24e2b784\"}]";
///
/// let events = extract_events(txt);
/// assert_eq!(events.len(), 1);
/// ```
pub fn extract_events(message: &str) -> Vec<Event> {
    let mut events = vec![];
    let json = serde_json::from_str::<serde_json::Value>(message);

    if json.is_err() {
        return events;
    }

    let json = json.unwrap();

    if !json.is_array() {
        return events;
    }

    let json = json.as_array().unwrap();

    for event in json {
        if !event.is_object() {
            continue;
        }

        let event = serde_json::from_value::<Event>(event.clone());

        if event.is_err() {
            continue;
        }

        events.push(event.unwrap());
    }

    events
}

/// Extract events from a websocket message
///
/// # Example
/// ```rust
/// use nostr_rust::events::extract_events_ws;
/// use tungstenite::Message;
///
/// let txt = "[\"EVENT\",\"deb0ab5bd829d1642c926b7897b078d027ca41870d0a499c1fd76e4b5af5ccbd\",{\"id\":\"f0382d932ddc5876bad3f9c5fdb84fb4c2af7ccefebfb491f13fbc47c38f8ae4\",\"kind\":1,\"pubkey\":\"884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6\",\"created_at\":1673131597,\"content\":\"Does anyone know a good crate rust to handle a Lightning node?\",\"tags\":[],\"sig\":\"53a629bae11dace9b487700cbe8e85058a3d7b6989e1e0bdd6eb4fb0201a3779742682f65ca37782c0cb93019a170e0a368bb033dfce1102df71420e24e2b784\"}]";
///
/// let events = extract_events_ws(&Message::Text(txt.to_string()));
/// assert_eq!(events.len(), 1);
/// ```
pub fn extract_events_ws(message: &crate::Message) -> Vec<Event> {
    if message.is_text() {
        return extract_events(message.to_text().unwrap());
    }

    vec![]
}

// Implementation of the NIP4 protocol
// https://github.com/nostr-protocol/nips/blob/master/04.md

// Thanks to Yuki Kishimoto for the inspiration with his module
// https://gitlab.com/p2kishimoto/nostr-rs-sdk/-/tree/master/crates/nostr-sdk-base

use crate::events::{Event, EventPrepare};
use crate::nips::nip4::NIP4Error;
use crate::nips::nip4::PrivateMessage;
use crate::nips::nip4::{decrypt, encrypt};
use crate::nostr_client::Client;
use crate::req::ReqFilter;
use crate::utils::get_timestamp;
use crate::Identity;
use aes::{cipher::KeyIvInit, Aes256};
use cbc::{Decryptor, Encryptor};
use std::str::FromStr;

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

impl Client {
    #[cfg(feature = "blocking")]
    /// Send private message to a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    ///
    /// client.send_private_message_blocking(&identity, pubkey, "Hello from Rust Nostr Client!", 0).unwrap();
    /// ```
    pub fn send_private_message_blocking(
        &mut self,
        identity: &Identity,
        hex_pubkey: &str,
        message: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP4Error> {
        let x_pub_key = secp256k1::XOnlyPublicKey::from_str(hex_pubkey)?;
        let encrypted_message = encrypt(&identity.secret_key, &x_pub_key, message)?;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 4,
            tags: vec![vec!["p".to_string(), hex_pubkey.to_string()]],
            content: encrypted_message,
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).unwrap();
        Ok(event)
    }

    #[cfg(feature = "blocking")]
    /// Get private events (messages) with a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    /// let messages = client.get_private_events_with_blocking(&identity, pubkey, 10).unwrap();
    /// ```
    pub fn get_private_events_with_blocking(
        &mut self,
        identity: &Identity,
        hex_pubkey: &str,
        limit: u64,
    ) -> Result<Vec<Event>, NIP4Error> {
        let events = self
            .get_events_of(vec![
                ReqFilter {
                    ids: None,
                    authors: Some(vec![identity.public_key_str.clone()]),
                    kinds: Some(vec![4]),
                    e: None,
                    p: Some(vec![hex_pubkey.to_string()]),
                    since: None,
                    until: None,
                    limit: Some(limit),
                },
                ReqFilter {
                    ids: None,
                    authors: Some(vec![hex_pubkey.to_string()]),
                    kinds: Some(vec![4]),
                    e: None,
                    p: Some(vec![identity.public_key_str.clone()]),
                    since: None,
                    until: None,
                    limit: Some(limit),
                },
            ])
            .unwrap();

        Ok(events)
    }

    #[cfg(feature = "blocking")]
    /// Get private messages with a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    /// let messages = client.get_private_messages_with_blocking(&identity, pubkey, 10).unwrap();
    /// ```
    pub fn get_private_messages_with_blocking(
        &mut self,
        identity: &Identity,
        hex_pubkey: &str,
        limit: u64,
    ) -> Result<Vec<PrivateMessage>, NIP4Error> {
        let x_pub_key = secp256k1::XOnlyPublicKey::from_str(hex_pubkey)?;
        let events =
            self.get_private_events_with(identity, x_pub_key.to_string().as_str(), limit)?;
        let mut messages: Vec<PrivateMessage> = vec![];

        for event in events {
            let decrypted_message = match decrypt(&identity.secret_key, &x_pub_key, &event.content)
            {
                Ok(message) => message,
                Err(_) => continue,
            };

            let private_message = PrivateMessage {
                author: event.pub_key,
                content: decrypted_message,
                timestamp: event.created_at,
            };

            messages.push(private_message);
        }

        // Sort messages by timestamp
        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        // Reverse order
        messages.reverse();

        Ok(messages)
    }
}

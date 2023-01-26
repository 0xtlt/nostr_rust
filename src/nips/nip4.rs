// Implementation of the NIP4 protocol
// https://github.com/nostr-protocol/nips/blob/master/04.md

// Thanks to Yuki Kishimoto for the inspiration with his module
// https://gitlab.com/p2kishimoto/nostr-rs-sdk/-/tree/master/crates/nostr-sdk-base

use crate::bech32::auto_bech32_to_hex;
use crate::events::{Event, EventPrepare};
use crate::nostr_client::Client;
use crate::req::ReqFilter;
use crate::utils::get_timestamp;
use crate::Identity;
use aes::{
    cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit},
    Aes256,
};
use base64::Engine;
use cbc::{Decryptor, Encryptor};
use secp256k1::{ecdh, rand::random, PublicKey, SecretKey, XOnlyPublicKey};
use serde::{Deserialize, Serialize};
use std::convert::From;
use std::str::FromStr;
use thiserror::Error;

type Aes256CbcEnc = Encryptor<Aes256>;
type Aes256CbcDec = Decryptor<Aes256>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateMessage {
    pub author: String,
    pub content: String,
    pub timestamp: u64,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error(
        r#"Invalid content format. Expected format "<encrypted_text>?iv=<initialization_vec>""#
    )]
    InvalidContentFormat,

    #[error("Error while decoding from base64")]
    Base64DecodeError,

    #[error("Error while encoding to UTF-8")]
    Utf8EncodeError,

    #[error("Wrong encryption block mode.The content must be encrypted using CBC mode!")]
    WrongBlockMode,

    #[error("Secp256k1 Error: {}", _0)]
    Secp256k1Error(#[from] secp256k1::Error),

    #[error("Bech32 Error: {}", _0)]
    Bech32Error(#[from] crate::bech32::Bech32Error),

    #[error("Error when decrypting the message")]
    DecryptionError,
}

/// Decrypt a private message
/// # Example
///
/// ```rust
/// // https://github.com/0xtlt/nostr_rust/issues/32
/// use nostr_rust::keys;
/// use nostr_rust::nips::nip4;
/// use secp256k1::XOnlyPublicKey;
/// use std::str::FromStr;
///
/// let id1 = keys::get_random_secret_key();
/// let id2 = keys::get_random_secret_key();
/// let id1pk = keys::normalize_public_key(&id1.1.to_string());
///
/// let event_content = "hello world!";
///
/// let system_sec_key = id2.0;
/// let sender_pub_key = XOnlyPublicKey::from_str(&id1pk).unwrap();
///
/// let message = nip4::encrypt(&system_sec_key, &sender_pub_key, event_content).unwrap();
/// let message = nip4::decrypt(&system_sec_key, &sender_pub_key, &message).unwrap();
/// ```
pub fn decrypt(
    sk: &SecretKey,
    pk: &XOnlyPublicKey,
    encrypted_content: &str,
) -> Result<String, Error> {
    let parsed_content: Vec<&str> = encrypted_content.split("?iv=").collect();
    if parsed_content.len() != 2 {
        return Err(Error::InvalidContentFormat);
    }

    let mut encrypted_content: Vec<u8> = base64::prelude::BASE64_STANDARD
        .decode(parsed_content[0])
        .unwrap()
        .to_vec();

    let iv: Vec<u8> = base64::prelude::BASE64_STANDARD
        .decode(parsed_content[1])
        .unwrap()
        .to_vec();
    let key: Vec<u8> = generate_shared_key(sk, pk)?;

    let key = key.as_slice().try_into();
    let iv = iv.as_slice().try_into();

    if key.is_err() || iv.is_err() {
        return Err(Error::Base64DecodeError);
    }

    let cipher = Aes256CbcDec::new(key.unwrap(), iv.unwrap());

    let result = cipher
        .decrypt_padded_mut::<Pkcs7>(&mut encrypted_content)
        .map_err(|_| Error::WrongBlockMode)?;

    String::from_utf8(result.try_into().unwrap()).map_err(|_| Error::Utf8EncodeError)
}

pub fn encrypt(sk: &SecretKey, pk: &XOnlyPublicKey, text: &str) -> Result<String, Error> {
    let key: Vec<u8> = generate_shared_key(sk, pk)?;
    let iv: [u8; 16] = random();

    let cipher = Aes256CbcEnc::new(key.as_slice().into(), &iv.into());
    let result: Vec<u8> = cipher.encrypt_padded_vec_mut::<Pkcs7>(text.as_bytes());

    Ok(format!(
        "{}?iv={}",
        base64::prelude::BASE64_STANDARD.encode(result),
        base64::prelude::BASE64_STANDARD.encode(iv)
    ))
}

fn generate_shared_key(sk: &SecretKey, pk: &XOnlyPublicKey) -> Result<Vec<u8>, Error> {
    let pk_normalized: PublicKey = from_schnorr_pk(pk)?;
    let ssp = ecdh::shared_secret_point(&pk_normalized, sk);

    let mut shared_key = [0u8; 32];
    shared_key.copy_from_slice(&ssp[..32]);
    Ok(shared_key.to_vec())
}

fn from_schnorr_pk(schnorr_pk: &XOnlyPublicKey) -> Result<PublicKey, Error> {
    let mut pk = String::from("02");
    pk.push_str(&schnorr_pk.to_string());

    Ok(PublicKey::from_str(&pk)?)
}

impl Client {
    #[cfg(not(feature = "async"))]
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
    /// client.send_private_message(&identity, pubkey, "Hello from Rust Nostr Client!", 0).unwrap();
    /// ```
    pub fn send_private_message(
        &mut self,
        identity: &Identity,
        pubkey: &str,
        message: &str,
        difficulty_target: u16,
    ) -> Result<Event, Error> {
        let hex_pubkey = auto_bech32_to_hex(pubkey)?;
        let x_pub_key = secp256k1::XOnlyPublicKey::from_str(&hex_pubkey)?;
        let encrypted_message = encrypt(&identity.secret_key, &x_pub_key, message)?;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 4,
            tags: vec![vec!["p".to_string(), hex_pubkey]],
            content: encrypted_message,
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).unwrap();
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Send private message to a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_send_private_message() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    ///     client.send_private_message(&identity, pubkey, "Hello from Rust Nostr Client!", 0).await.unwrap();
    /// }
    /// ```
    pub async fn send_private_message(
        &mut self,
        identity: &Identity,
        pubkey: &str,
        message: &str,
        difficulty_target: u16,
    ) -> Result<Event, Error> {
        let hex_pubkey = auto_bech32_to_hex(pubkey)?;
        let x_pub_key = secp256k1::XOnlyPublicKey::from_str(&hex_pubkey)?;
        let encrypted_message = encrypt(&identity.secret_key, &x_pub_key, message)?;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 4,
            tags: vec![vec!["p".to_string(), hex_pubkey.to_string()]],
            content: encrypted_message,
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await.unwrap();
        Ok(event)
    }

    #[cfg(not(feature = "async"))]
    /// Get private events (messages) with a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    /// let messages = client.get_private_events_with(&identity, pubkey, 10).unwrap();
    /// ```
    pub fn get_private_events_with(
        &mut self,
        identity: &Identity,
        pubkey: &str,
        limit: u64,
    ) -> Result<Vec<Event>, Error> {
        let hex_pubkey = &auto_bech32_to_hex(pubkey)?;

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

    #[cfg(feature = "async")]
    /// Get private events (messages) with a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_get_private_events_with() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    ///     client.get_private_events_with(&identity, pubkey, 10).await.unwrap();
    /// }
    /// ```
    pub async fn get_private_events_with(
        &mut self,
        identity: &Identity,
        pubkey: &str,
        limit: u64,
    ) -> Result<Vec<Event>, Error> {
        let hex_pubkey = &auto_bech32_to_hex(pubkey)?;

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
            .await
            .unwrap();

        Ok(events)
    }

    #[cfg(not(feature = "async"))]
    /// Get private messages with a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    /// let messages = client.get_private_messages_with(&identity, pubkey, 10).unwrap();
    /// ```
    pub fn get_private_messages_with(
        &mut self,
        identity: &Identity,
        pubkey: &str,
        limit: u64,
    ) -> Result<Vec<PrivateMessage>, Error> {
        let hex_pubkey = &auto_bech32_to_hex(pubkey)?;

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

    #[cfg(feature = "async")]
    /// Get private messages with a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_get_private_messages_with() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    ///     let messages = client.get_private_messages_with(&identity, pubkey, 10).await.unwrap();
    /// }
    /// ```
    pub async fn get_private_messages_with(
        &mut self,
        identity: &Identity,
        pubkey: &str,
        limit: u64,
    ) -> Result<Vec<PrivateMessage>, Error> {
        let hex_pubkey = &auto_bech32_to_hex(pubkey)?;

        let x_pub_key = secp256k1::XOnlyPublicKey::from_str(hex_pubkey)?;
        let events = self
            .get_private_events_with(identity, x_pub_key.to_string().as_str(), limit)
            .await?;
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

    // TODO: get a list of private messages with a list of public keys
}

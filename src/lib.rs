use events::{Event, EventPrepare};
use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;
use utils::get_timestamp;

pub mod bech32;
pub mod events;
pub mod keys;
pub mod nips;
pub mod nostr_client;
pub mod req;
pub mod utils;
pub mod websocket;

pub const DEFAULT_HASHTAG: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub type Message = tungstenite::Message;

/// Nostr Identity with secret and public keys
pub struct Identity {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub public_key_str: String,
    pub address: String,
}

impl Identity {
    /// Make event and return it
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    ///
    /// #[cfg(feature = "async")]
    /// async fn test_make_event() {
    ///    let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///    let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///    let event = identity.make_event(1, "Hello Nostr!", &vec![], 0);
    ///
    ///    assert_eq!(event.kind, 1);
    ///    assert_eq!(event.content, "Hello Nostr!");
    ///    assert_eq!(event.tags.len(), 0);
    /// }
    ///
    /// #[cfg(not(feature = "async"))]
    /// fn test_make_event() {
    ///    let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    ///    let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///    let event = identity.make_event(1, "Hello Nostr!", &vec![], 0);
    ///
    ///    assert_eq!(event.kind, 1);
    ///    assert_eq!(event.content, "Hello Nostr!");
    ///    assert_eq!(event.tags.len(), 0);
    /// }
    ///
    /// #[cfg(feature = "async")]
    /// tokio::runtime::Runtime::new().unwrap().block_on(test_make_event());
    ///
    /// #[cfg(not(feature = "async"))]
    /// test_make_event();
    /// ```
    pub fn make_event(
        &self,
        kind: u16,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Event {
        EventPrepare {
            pub_key: self.public_key_str.clone(),
            created_at: get_timestamp(),
            kind,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(self, difficulty_target)
    }
}

impl FromStr for Identity {
    type Err = String;

    /// Create an Identity from a secret key as a hex string
    /// # Example
    /// ```
    /// use nostr_rust::Identity;
    /// use std::str::FromStr;
    ///
    /// // Working format
    /// let identity = Identity::from_str(env!("SECRET_KEY"));
    /// assert!(identity.is_ok());
    ///
    /// // Invalid format
    /// let identity = Identity::from_str("aeaeaeaeae");
    /// assert!(identity.is_err());
    /// ```
    fn from_str(secret_key: &str) -> Result<Self, Self::Err> {
        let secret_key = keys::secret_key_from_str(&{
            if secret_key.starts_with("nsec") {
                crate::bech32::from_hb_to_hex(crate::bech32::ToBech32Kind::SecretKey, secret_key)
                    .unwrap()
            } else {
                secret_key.to_string()
            }
        })?;
        let public_key = keys::get_public_key_from_secret(&secret_key);
        let address = keys::get_str_keys_from_secret(&secret_key).1;

        Ok(Self {
            secret_key,
            public_key,
            public_key_str: address.to_string(),
            address,
        })
    }
}

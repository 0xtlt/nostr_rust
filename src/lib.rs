use secp256k1::{PublicKey, SecretKey};
use std::str::FromStr;

pub mod events;
pub mod keys;
pub mod nips;
pub mod nostr_client;
pub mod req;
pub mod utils;
pub mod websocket;

/// Nostr Identity with secret and public keys
pub struct Identity {
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    pub public_key_str: String,
    pub address: String,
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
        let secret_key = keys::secret_key_from_str(secret_key)?;
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

use crate::{events::EventPrepare, nostr_client::Client, utils::get_timestamp, Identity};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use secp256k1::{ecdh::SharedSecret, rand::random, PublicKey};
use std::str::FromStr;

// Implementation of the NIP4 protocol
// https://github.com/nostr-protocol/nips/blob/master/04.md

impl Client {
    pub fn get_shared_identity(identity: &Identity, hex_pubkey: &str) -> SharedSecret {
        SharedSecret::new(
            &PublicKey::from_str(&format!("02{}", hex_pubkey)).unwrap(),
            &identity.secret_key,
        )
    }

    /// Send private message to a public key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let pubkey = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    ///
    /// client.send_private_message(&identity, pubkey, "Hello from Rust Nostr Client!").unwrap();
    /// ```
    pub fn send_private_message(
        &mut self,
        identity: &Identity,
        hex_pubkey: &str,
        message: &str,
    ) -> Result<(), String> {
        let shared_secret = Client::get_shared_identity(identity, hex_pubkey)
            .display_secret()
            .to_string();

        let iv: [u8; 16] = random();
        let iv_str = iv.iter().map(|b| format!("{:02x}", b)).collect::<String>();

        let magic = MagicCrypt256::new(shared_secret, Some(iv_str));

        let encrypted_message = magic.encrypt_to_base64(message);

        let content = format!("{}?iv={}", encrypted_message, base64::encode(iv));

        println!("content: {}", content);

        // Actually working on it
        // Set the condition to true if you want to send the message
        if false {
            let event = EventPrepare {
                pub_key: identity.public_key_str.clone(),
                created_at: get_timestamp(),
                kind: 4,
                tags: vec![vec!["p".to_string(), hex_pubkey.to_string()]],
                content,
            }
            .to_event(identity);

            self.publish_event(&event)?;
        }
        Ok(())
    }
}

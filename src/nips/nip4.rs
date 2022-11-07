use crate::{events::EventPrepare, nostr_client::Client, utils::get_timestamp, Identity};
use libaes::Cipher;
use rand::{rngs::OsRng, Rng};
use secp256k1::{ecdh::SharedSecret, PublicKey, SecretKey};
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

        let iv: [u8; 16] = OsRng.gen::<[u8; 16]>();
        let key: [u8; 32] = hex::decode(shared_secret).unwrap().try_into().unwrap();
        let cipher = Cipher::new_256(&key);

        let encrypted = cipher.cbc_encrypt(&iv, message.as_bytes());
        let content = format!("{}?iv={}", base64::encode(encrypted), base64::encode(iv));

        println!("content: {}", content);

        todo!("Not working yet on Nostr server side");

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 4,
            tags: vec![vec!["p".to_string(), hex_pubkey.to_string()]],
            content,
        }
        .to_event(identity);

        self.publish_event(&event)?;
        Ok(())
    }
}

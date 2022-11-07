use crate::{events::EventPrepare, nostr_client::Client, utils::get_timestamp, Identity};
use aes::cipher::{block_padding::Pkcs7, generic_array::GenericArray, BlockEncryptMut, KeyIvInit};
use rand::{rngs::OsRng, RngCore};
use secp256k1::{ecdh::SharedSecret, PublicKey};
use std::str::FromStr;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
// type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

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
        let message = message.as_bytes();
        let shared_secret = Client::get_shared_identity(identity, hex_pubkey)
            .display_secret()
            .to_string();

        let hex_key = hex::decode(shared_secret).unwrap();

        let mut iv = [0u8; 16];
        OsRng.fill_bytes(&mut iv);
        let key: &GenericArray<u8, generic_array::typenum::U32> =
            GenericArray::from_slice(&hex_key);

        // buffer must be big enough for padded plaintext
        let mut encrypted = vec![0u8; message.len() + 16];

        // Length of the message
        let pt_len = message.len();

        // Add padding
        // let mut padded = message.to_vec();
        // let padding = 16 - (pt_len % 16);
        // padded.extend(vec![padding as u8; padding]);

        // encrypted[..padded.len()].copy_from_slice(&padded);

        // Put the message in the buffer
        encrypted[..pt_len].copy_from_slice(message);

        let ct = Aes256CbcEnc::new(key, &iv.into())
            .encrypt_padded_mut::<Pkcs7>(&mut encrypted, pt_len)
            .unwrap();

        let content = format!("{}?iv={}", base64::encode(ct), base64::encode(iv));
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

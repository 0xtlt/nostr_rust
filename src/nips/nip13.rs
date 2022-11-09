use crate::{
    events::{Event, EventPrepare},
    nostr_client::{Client, ClientError},
    utils::{self, get_timestamp},
    Identity,
};
use hex::FromHexError;
use rand::Rng;
use secp256k1::{KeyPair, SECP256K1};
use thiserror::Error;

// Implementation of the NIP13 protocol
// https://github.com/nostr-protocol/nips/blob/master/13.md

#[derive(Error, Debug, PartialEq)]
pub enum NIP13Error {
    #[error("Content Id is invalid")]
    InvalidContentId(FromHexError),

    #[error("The client has an error")]
    ClientError(ClientError),
}

impl From<ClientError> for NIP13Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

impl From<FromHexError> for NIP13Error {
    fn from(err: FromHexError) -> Self {
        Self::InvalidContentId(err)
    }
}

impl EventPrepare {
    /// Counts leading zero bits to calculate PoW difficulty
    /// # Example
    /// ```rust
    /// use nostr_rust::{events::EventPrepare};
    /// let hash = hex::decode("000000000e9d97a1ab09fc381030b346cdd7a142ad57e6df0b46dc9bef6c7e2d")
    ///        .unwrap();
    ///
    /// let diff = EventPrepare::count_leading_zero_bits(hash);
    /// assert_eq!(diff, 36)
    /// ```
    pub fn count_leading_zero_bits(content_id: Vec<u8>) -> u32 {
        let mut total: u32 = 0;

        for c in content_id {
            let bits = c.leading_zeros();
            total += bits;
            if bits != 8 {
                break;
            }
        }
        total
    }

    /// Transfrom event to NostrEvent with Proof of Work
    /// # Example
    /// ```rust
    /// use std::str::FromStr;
    /// use nostr_rust::{events::EventPrepare, Identity};
    ///
    /// let event = EventPrepare {
    ///  pub_key: env!("PUBLIC_KEY").to_string(),
    ///  created_at: 0, // Don't use this in production
    ///  kind: 0,
    ///  tags: vec![],
    ///  content: "content".to_string(),
    /// };
    ///
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let difficulty = 10;
    /// let nostr_event = event.to_pow_event(&identity, difficulty);
    /// let event_id = hex::decode(nostr_event.id).unwrap();
    /// let event_diffculty = count_leading_zero_bits(event_id);
    /// assert_gt!(event_diffculty, difficulty);
    /// assert_eq!(nostr_event.content, "content");
    /// assert_eq!(nostr_event.kind, 0);
    /// assert_eq!(nostr_event.tags.len(), 1);
    /// assert_eq!(nostr_event.created_at, 0);
    /// assert_eq!(nostr_event.pub_key, env!("PUBLIC_KEY"));
    /// assert_eq!(nostr_event.sig.len(), 128);
    /// ```
    pub fn to_pow_event(
        &mut self,
        secret_key: &Identity,
        difficulty: u32,
    ) -> Result<Event, NIP13Error> {
        let mut rng = rand::thread_rng();
        loop {
            let nouce: u32 = rng.gen_range(0..999999);

            self.tags.push(vec![
                "nouce".to_string(),
                nouce.to_string(),
                difficulty.to_string(),
            ]);

            let content_id = self.get_content_id();
            let content_id = hex::decode(content_id)?;

            if Self::count_leading_zero_bits(content_id) >= difficulty {
                break;
            }

            self.tags.pop();
            self.created_at = utils::get_timestamp();
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

        Ok(Event {
            id: self.get_content_id(),
            pub_key: self.pub_key.clone(),
            created_at: self.created_at,
            kind: self.kind,
            tags: self.tags.clone(),
            content: self.content.clone(),
            sig: signature,
        })
    }
}

impl Client {
    /// Publish a text note event with Proof of Work
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, utils::get_timestamp};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let message = format!("Hello Nostr! {}", get_timestamp());
    /// client.publish_pow_text_note(&identity, &message, &vec![], 10).unwrap();
    /// ```
    pub fn publish_pow_text_note(
        &mut self,
        identity: &Identity,
        content: &str,
        tags: &[Vec<String>],
        difficulty: u32,
    ) -> Result<Event, NIP13Error> {
        let mut event_prepare = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 1,
            tags: tags.to_vec(),
            content: content.to_string(),
        };

        let event = event_prepare.to_pow_event(identity, difficulty)?;

        self.publish_event(&event)?;

        Ok(event)
    }
}

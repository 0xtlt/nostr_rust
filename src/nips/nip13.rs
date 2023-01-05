use crate::{events::EventPrepare, nostr_client::ClientError, utils::get_timestamp};
use hex::FromHexError;
use rand::Rng;
use thiserror::Error;

// Implementation of the NIP13 protocol
// https://github.com/nostr-protocol/nips/blob/master/13.md

#[derive(Error, Debug)]
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
    pub fn count_leading_zero_bits(content_id: Vec<u8>) -> u16 {
        let mut total: u16 = 0;

        for c in content_id {
            let bits = c.leading_zeros() as u16;
            total += bits;
            if bits != 8 {
                break;
            }
        }
        total
    }

    /// Transform event to NostrEvent with Proof of Work
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
    /// let difficulty = 10;
    /// event.to_pow_event(difficulty).unwrap();
    /// let event_id = event.get_content_id();
    /// let event_id = hex::decode(event_id).unwrap();
    /// let event_difficulty = EventPrepare::count_leading_zero_bits(event_id);
    /// assert!(event_difficulty >= difficulty.into());
    /// assert_eq!(event.content, "content");
    /// assert_eq!(event.kind, 0);
    /// assert_eq!(event.tags.len(), 1);
    /// assert!(event.created_at > 0);
    /// assert_eq!(event.pub_key, env!("PUBLIC_KEY"));
    ///
    /// ```
    pub fn to_pow_event(&mut self, difficulty: u16) -> Result<(), NIP13Error> {
        let mut rng = rand::thread_rng();
        loop {
            let nonce: u32 = rng.gen_range(0..999999);

            self.tags.push(vec![
                "nonce".to_string(),
                nonce.to_string(),
                difficulty.to_string(),
            ]);

            let content_id = self.get_content_id();
            let content_id = hex::decode(content_id)?;

            if Self::count_leading_zero_bits(content_id) >= difficulty {
                break;
            }

            self.tags.pop();
            self.created_at = get_timestamp();
        }

        Ok(())
    }
}

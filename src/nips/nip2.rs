use crate::nostr_client::ClientError;
use thiserror::Error;

// Implementation of the NIP2 protocol
// https://github.com/nostr-protocol/nips/blob/master/02.md

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NIP2Error {
    #[error("The client has an error")]
    ClientError(ClientError),
}

impl From<ClientError> for NIP2Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

#[derive(Debug, Clone)]
pub struct ContactListTag {
    /// 32-bytes hex key - the public key of the contact
    pub key: String,
    /// main relay URL
    pub main_relay: Option<String>,
    /// Petname - surname
    pub surname: Option<String>,
}

impl ContactListTag {
    pub fn to_tags(&self) -> Vec<String> {
        let mut tags: Vec<String> = vec![String::from("p"), self.key.clone()];

        if let Some(main_relay) = &self.main_relay {
            tags.push(main_relay.clone());

            if let Some(surname) = &self.surname {
                tags.push(surname.clone());
            }
        } else if self.surname.is_some() {
            tags.push(String::from(""));

            tags.push(self.surname.clone().unwrap());
        }

        tags
    }
}

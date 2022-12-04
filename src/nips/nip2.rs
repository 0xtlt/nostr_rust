use crate::{
    events::EventPrepare,
    nostr_client::{Client, ClientError},
    req::ReqFilter,
    utils::get_timestamp,
    Identity,
};
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

impl Client {
    #[cfg(not(feature = "async"))]
    /// Set the contact list of the identity
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, nips::nip2::ContactListTag};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the contact list of the identity
    /// client.set_contact_list(&identity, vec![ContactListTag {
    ///   key: "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///   main_relay: Some(env!("RELAY_URL").to_string()),
    ///   surname: Some("Rust Nostr Client".to_string()),
    /// }],
    /// 0).unwrap();
    /// ```
    pub fn set_contact_list(
        &mut self,
        identity: &Identity,
        contact_list: Vec<ContactListTag>,
        difficulty_target: u16,
    ) -> Result<(), NIP2Error> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 3,
            tags: contact_list
                .iter()
                .map(|contact| contact.to_tags())
                .collect(),
            content: String::new(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event)?;
        Ok(())
    }

    #[cfg(feature = "async")]
    /// Set the contact list of the identity
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, nips::nip2::ContactListTag};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the contact list of the identity
    /// let res = async {
    ///   client.set_contact_list(&identity, vec![ContactListTag {
    ///     key: "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///     main_relay: Some(env!("RELAY_URL").to_string()),
    ///     surname: Some("Rust Nostr Client".to_string()),
    ///   }],
    ///   0).await;
    /// }
    ///
    /// res.await?;
    /// ```
    pub async fn set_contact_list(
        &mut self,
        identity: &Identity,
        contact_list: Vec<ContactListTag>,
        difficulty_target: u16,
    ) -> Result<(), NIP2Error> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 3,
            tags: contact_list
                .iter()
                .map(|contact| contact.to_tags())
                .collect(),
            content: String::new(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await?;
        Ok(())
    }

    #[cfg(not(feature = "async"))]
    /// Get the contact list of a pub key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, nips::nip2::ContactListTag};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let contact_list = client.get_contact_list("884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6").unwrap();
    /// ```
    pub fn get_contact_list(&mut self, pubkey: &str) -> Result<Vec<ContactListTag>, NIP2Error> {
        let mut contact_list: Vec<ContactListTag> = vec![];

        let events = self.get_events_of(vec![ReqFilter {
            ids: None,
            authors: Some(vec![pubkey.to_string()]),
            kinds: Some(vec![3]),
            e: None,
            p: None,
            since: None,
            until: None,
            limit: Some(1),
        }])?;

        for event in events {
            for tag in event.tags {
                if tag[0] == "p" {
                    let mut contact = ContactListTag {
                        key: tag[1].clone(),
                        main_relay: None,
                        surname: None,
                    };

                    if tag.len() > 2 {
                        contact.main_relay = Some(tag[2].clone());

                        if tag.len() > 3 {
                            contact.surname = Some(tag[3].clone());
                        }
                    }

                    contact_list.push(contact);
                }
            }
        }

        Ok(contact_list)
    }

    #[cfg(feature = "async")]
    /// Get the contact list of a pub key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, nips::nip2::ContactListTag};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let res = async {
    ///   let contact_list = client.get_contact_list("884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6").await;
    /// }
    ///
    /// let contact_list = res.await?;
    /// ```
    pub async fn get_contact_list(
        &mut self,
        pubkey: &str,
    ) -> Result<Vec<ContactListTag>, NIP2Error> {
        let mut contact_list: Vec<ContactListTag> = vec![];

        let events = self
            .get_events_of(vec![ReqFilter {
                ids: None,
                authors: Some(vec![pubkey.to_string()]),
                kinds: Some(vec![3]),
                e: None,
                p: None,
                since: None,
                until: None,
                limit: Some(1),
            }])
            .await?;

        for event in events {
            for tag in event.tags {
                if tag[0] == "p" {
                    let mut contact = ContactListTag {
                        key: tag[1].clone(),
                        main_relay: None,
                        surname: None,
                    };

                    if tag.len() > 2 {
                        contact.main_relay = Some(tag[2].clone());

                        if tag.len() > 3 {
                            contact.surname = Some(tag[3].clone());
                        }
                    }

                    contact_list.push(contact);
                }
            }
        }

        Ok(contact_list)
    }
}

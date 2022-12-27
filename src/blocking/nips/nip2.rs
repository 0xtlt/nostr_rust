use crate::events::EventPrepare;
use crate::nips::nip2::{ContactListTag, NIP2Error};
use crate::nostr_client::Client;
use crate::req::ReqFilter;
use crate::utils::get_timestamp;
use crate::Identity;

// Implementation of the NIP2 protocol
// https://github.com/nostr-protocol/nips/blob/master/02.md

impl Client {
    #[cfg(feature = "blocking")]
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
    /// client.set_contact_list_blocking(&identity, vec![ContactListTag {
    ///   key: "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///   main_relay: Some(env!("RELAY_URL").to_string()),
    ///   surname: Some("Rust Nostr Client".to_string()),
    /// }],
    /// 0).unwrap();
    /// ```
    pub fn set_contact_list_blocking(
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

    #[cfg(feature = "blocking")]
    /// Get the contact list of a pub key
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, nips::nip2::ContactListTag};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let contact_list = client.get_contact_list_blocking("884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6").unwrap();
    /// ```
    pub fn get_contact_list_blocking(
        &mut self,
        pubkey: &str,
    ) -> Result<Vec<ContactListTag>, NIP2Error> {
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
}

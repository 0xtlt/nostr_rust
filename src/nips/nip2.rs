use crate::{events::EventPrepare, nostr_client::Client, utils::get_timestamp, Identity};

// Implementation of the NIP2 protocol
// https://github.com/nostr-protocol/nips/blob/master/02.md

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
    /// Set the contact list of the identity
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, nips::nip2::ContactListTag};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the contact list of the identity
    /// client.set_contact_list(&identity, vec![ContactListTag {
    ///   key: "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    ///   main_relay: Some("wss://nostr-pub.wellorder.net".to_string()),
    ///   surname: Some("Rust Nostr Client".to_string()),
    /// }]).unwrap();
    /// ```
    pub fn set_contact_list(
        &mut self,
        identity: &Identity,
        contact_list: Vec<ContactListTag>,
    ) -> Result<(), String> {
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
        .to_event(identity);

        self.publish_event(&event)?;
        Ok(())
    }
}

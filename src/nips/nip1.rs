use crate::{
    events::{Event, EventPrepare},
    nostr_client::Client,
    utils::get_timestamp,
    Identity,
};
use serde_json::json;

// Implementation of the NIP1 protocol
// https://github.com/nostr-protocol/nips/blob/master/01.md

impl Client {
    /// Set the metadata of the identity
    /// # Example
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the metadata of the identity but not the profile picture one
    /// client.set_metadata(&identity, Some("Rust Nostr Client"), Some("Automated account for Rust Nostr Client tests :)"), None).unwrap();
    /// ```
    pub fn set_metadata(
        &mut self,
        identity: &Identity,
        name: Option<&str>,
        about: Option<&str>,
        picture: Option<&str>,
    ) -> Result<(), String> {
        let mut json_body = json!({});

        if name.is_none() && about.is_none() && picture.is_none() {
            return Err("No metadata provided".to_string());
        }

        if let Some(name) = name {
            json_body["name"] = json!(name);
        }

        if let Some(about) = about {
            json_body["about"] = json!(about);
        }

        if let Some(picture) = picture {
            json_body["picture"] = json!(picture);
        }

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 0,
            tags: vec![],
            content: json_body.to_string(),
        }
        .to_event(identity);

        self.publish_event(&event)?;
        Ok(())
    }

    /// Publish a text note (text_note) event
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, utils::get_timestamp};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let message = format!("Hello Nostr! {}", get_timestamp());
    /// client.publish_text_note(&identity, &message, &vec![]).unwrap();
    /// ```
    pub fn publish_text_note(
        &mut self,
        identity: &Identity,
        content: &str,
        tags: &[Vec<String>],
    ) -> Result<Event, String> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 1,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(identity);

        self.publish_event(&event)?;
        Ok(event)
    }

    /// Add recommended relay server
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec!["wss://nostr-pub.wellorder.net"]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the recommended relay server to the one hosted by Wellorder
    /// client.add_recommended_relay(&identity, "wss://relay.damus.io").unwrap();
    /// ```
    pub fn add_recommended_relay(
        &mut self,
        identity: &Identity,
        relay: &str,
    ) -> Result<(), String> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 2,
            tags: vec![],
            content: relay.to_string(),
        }
        .to_event(identity);

        self.publish_event(&event)?;
        Ok(())
    }
}

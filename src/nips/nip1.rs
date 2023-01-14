use crate::{
    events::{Event, EventPrepare},
    nostr_client::{Client, ClientError},
    utils::get_timestamp,
    Identity,
};
use serde_json::json;
use thiserror::Error;

use super::nip5::NIP5Error;

// Implementation of the NIP1 protocol
// https://github.com/nostr-protocol/nips/blob/master/01.md

#[derive(Error, Debug)]
pub enum NIP1Error {
    #[error("No metadata given")]
    NoMetadata,

    #[error("The client has an error")]
    ClientError(#[from] ClientError),

    #[error("NIP05 error")]
    NIP05Error(#[from] NIP5Error),

    #[error("Given NIP05 is invalid with the given pubkey")]
    BadNIP05,
}

impl Client {
    /// Set the metadata of the identity asynchronously
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// async fn test_set_metadata() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    ///     client.set_metadata(&identity, Some("Rust Nostr Client"), Some("Automated account for Rust Nostr Client tests :)"), None, 0).await.unwrap();
    /// }
    ///
    /// tokio::runtime::Runtime::new().unwrap().block_on(test_set_metadata());
    /// ```
    pub async fn set_metadata(
        &mut self,
        identity: &Identity,
        name: Option<&str>,
        about: Option<&str>,
        picture: Option<&str>,
        difficulty_target: u16,
    ) -> Result<Event, NIP1Error> {
        let mut json_body = json!({});

        if name.is_none() && about.is_none() && picture.is_none() {
            return Err(NIP1Error::NoMetadata);
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
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await?;
        Ok(event)
    }

    /// Broadcast event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, utils::get_timestamp};
    /// use std::str::FromStr;
    /// use serde_json::json;
    ///
    /// async fn test_broadcast_event() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    ///    let event = identity.make_event(1, "Hello Nostr!", &vec![], 0);
    ///    client.broadcast_event(&event).await.unwrap();
    /// }
    ///
    /// tokio::runtime::Runtime::new().unwrap().block_on(test_broadcast_event());
    /// ```
    pub async fn broadcast_event(&mut self, event: &Event) -> Result<(), NIP1Error> {
        self.publish_event(event).await?;
        Ok(())
    }

    /// Publish a text note (text_note) event asynchronously
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, utils::get_timestamp};
    /// use std::str::FromStr;
    ///
    /// async fn test_publish_text_note() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     let message = format!("Hello Nostr! {}", get_timestamp());
    ///     client.publish_text_note(&identity, &message, &vec![], 0).await.unwrap();
    /// }
    ///
    /// tokio::runtime::Runtime::new().unwrap().block_on(test_publish_text_note());
    /// ```
    pub async fn publish_text_note(
        &mut self,
        identity: &Identity,
        content: &str,
        tags: &[Vec<String>],
        difficulty_target: u16,
    ) -> Result<Event, NIP1Error> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 1,
            tags: tags.to_vec(),
            content: content.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await?;
        Ok(event)
    }

    /// Add recommended relay server asynchronously
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_add_recommended_relay() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     client.add_recommended_relay(&identity, "wss://relay.damus.io", 0).await.unwrap();
    /// }
    ///
    /// tokio::runtime::Runtime::new().unwrap().block_on(test_add_recommended_relay());
    /// ```
    pub async fn add_recommended_relay(
        &mut self,
        identity: &Identity,
        relay: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP1Error> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 2,
            tags: vec![],
            content: relay.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await?;
        Ok(event)
    }
}

use crate::{
    events::{Event, EventPrepare},
    nostr_client::{Client, ClientError},
    utils::get_timestamp,
    Identity,
};
use serde_json::json;
use thiserror::Error;

// Implementation of the NIP1 protocol
// https://github.com/nostr-protocol/nips/blob/master/01.md

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NIP1Error {
    #[error(r#"No metadata given"#)]
    NoMetadata,

    #[error("The client has an error")]
    ClientError(ClientError),
}

impl From<ClientError> for NIP1Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

impl Client {
    #[cfg(not(feature = "async"))]
    /// Set the metadata of the identity
    /// # Example
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the metadata of the identity but not the profile picture one
    /// client.set_metadata(&identity, Some("Rust Nostr Client"), Some("Automated account for Rust Nostr Client tests :)"), None, 0).unwrap();
    /// ```
    pub fn set_metadata(
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

        self.publish_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Set the metadata of the identity asynchronously
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// use async_std::task;
    ///
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// task::spawn(async move {
    ///     // Here we set the metadata of the identity asynchronously
    ///     client.set_metadata(&identity, Some("Rust Nostr Client"), Some("Automated account for Rust Nostr Client tests :)"), None, 0).await.unwrap();
    /// });
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

    #[cfg(not(feature = "async"))]
    /// Publish a text note (text_note) event
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, utils::get_timestamp};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let message = format!("Hello Nostr! {}", get_timestamp());
    /// client.publish_text_note(&identity, &message, &vec![], 0).unwrap();
    /// ```
    pub fn publish_text_note(
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

        self.publish_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Publish a text note (text_note) event asynchronously
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity, utils::get_timestamp};
    /// use std::str::FromStr;
    /// use async_std::task;
    ///
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// let message = format!("Hello Nostr! {}", get_timestamp());
    ///
    /// task::spawn(async move {
    ///     // Here we publish the text note asynchronously
    ///     client.publish_text_note(&identity, &message, &vec![], 0).await.unwrap();
    /// });
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

    #[cfg(not(feature = "async"))]
    /// Add recommended relay server
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we set the recommended relay server to the url set in env
    /// client.add_recommended_relay(&identity, "wss://relay.damus.io", 0).unwrap();
    /// ```
    pub fn add_recommended_relay(
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

        self.publish_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Add recommended relay server asynchronously
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// use async_std::task;
    ///
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// task::spawn(async move {
    ///     // Here we add the recommended relay server asynchronously
    ///     client.add_recommended_relay(&identity, "wss://relay.damus.io", 0).await.unwrap();
    /// });
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

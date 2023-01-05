use crate::{
    events::{Event, EventPrepare},
    nostr_client::{Client, ClientError},
    utils::get_timestamp,
    Identity,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NIP9Error {
    #[error("The client has an error")]
    ClientError(ClientError),
}

impl From<ClientError> for NIP9Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

impl Client {
    #[cfg(not(feature = "async"))]
    /// Delete an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// // Create an event
    /// let event = client.publish_text_note(&identity, "Hello Nostr! :)", &[], 0)
    ///   .unwrap();
    ///
    /// // Delete the event
    /// client.delete_event(&identity, &event.id, 0).unwrap();
    /// ```
    pub fn delete_event(
        &mut self,
        identity: &Identity,
        event_id: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP9Error> {
        self.delete_event_with_reason(identity, event_id, "", difficulty_target)
    }

    #[cfg(feature = "async")]
    /// Delete an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_delete_event() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     // Create an event
    ///     let event = client.publish_text_note(&identity, "Hello Nostr! :)", &[], 0)
    ///   .await.unwrap();
    ///
    ///     // Delete the event
    ///     client.delete_event(&identity, &event.id, 0).await.unwrap();
    /// }
    /// ```
    pub async fn delete_event(
        &mut self,
        identity: &Identity,
        event_id: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP9Error> {
        self.delete_event_with_reason(identity, event_id, "", difficulty_target)
            .await
    }

    #[cfg(not(feature = "async"))]
    /// Delete an event with a reason
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// // Create an event
    /// let event = client.publish_text_note(&identity, "Hello Nostr! :)", &[], 0)
    ///  .unwrap();
    ///
    /// // Delete the event with a reason
    /// client.delete_event_with_reason(&identity, &event.id, "This is a reason", 0).unwrap();
    /// ```
    pub fn delete_event_with_reason(
        &mut self,
        identity: &Identity,
        event_id: &str,
        reason: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP9Error> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 5,
            tags: vec![vec!["e".to_string(), event_id.to_string()]],
            content: reason.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// Delete an event with a reason
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_delete_event_with_reason() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     // Create an event
    ///     let event = client.publish_text_note(&identity, "Hello Nostr! :)", &[], 0)
    ///  .await.unwrap();
    ///
    ///     // Delete the event with a reason
    ///     client.delete_event_with_reason(&identity, &event.id, "This is a reason", 0).await.unwrap();
    /// }
    /// ```
    pub async fn delete_event_with_reason(
        &mut self,
        identity: &Identity,
        event_id: &str,
        reason: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP9Error> {
        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 5,
            tags: vec![vec!["e".to_string(), event_id.to_string()]],
            content: reason.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await?;
        Ok(event)
    }
}

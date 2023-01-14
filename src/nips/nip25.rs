use crate::bech32::auto_bech32_to_hex;
use crate::{
    events::{Event, EventPrepare},
    nostr_client::{Client, ClientError},
    utils::get_timestamp,
    Identity,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NIP25Error {
    #[error("The client has an error")]
    ClientError(ClientError),

    #[error("Bech32 Error: {}", _0)]
    Bech32Error(#[from] crate::bech32::Bech32Error),
}

impl From<ClientError> for NIP25Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

impl Client {
    #[cfg(not(feature = "async"))]
    /// React to an event
    ///
    /// '+' = Like\
    /// '-' = Dislike\
    /// Emoji = React with an emoji
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    /// // Here we react to an event
    /// client.react_to(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", "+", 0).unwrap();
    /// ```
    pub fn react_to(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        reaction: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP25Error> {
        let hex_id = auto_bech32_to_hex(event_id)?;
        let hex_pk = auto_bech32_to_hex(event_pub_key)?;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 7,
            tags: vec![vec!["e".to_string(), hex_id], vec!["p".to_string(), hex_pk]],
            content: reaction.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event)?;
        Ok(event)
    }

    #[cfg(feature = "async")]
    /// React to an event
    ///
    /// '+' = Like\
    /// '-' = Dislike\
    /// Emoji = React with an emoji
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_react_to() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///
    ///     // Here we react to an event
    ///     client.react_to(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", "+", 0).await.unwrap();
    /// }
    /// ```
    pub async fn react_to(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        reaction: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP25Error> {
        let hex_id = auto_bech32_to_hex(event_id)?;
        let hex_pk = auto_bech32_to_hex(event_pub_key)?;

        let event = EventPrepare {
            pub_key: identity.public_key_str.clone(),
            created_at: get_timestamp(),
            kind: 7,
            tags: vec![vec!["e".to_string(), hex_id], vec!["p".to_string(), hex_pk]],
            content: reaction.to_string(),
        }
        .to_event(identity, difficulty_target);

        self.publish_event(&event).await?;
        Ok(event)
    }

    #[cfg(not(feature = "async"))]
    /// Add a like to an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// client.like(&identity, "342060554ca30a9792f6e6959675ae734aed0223e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", 0).unwrap();
    /// ```
    pub fn like(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP25Error> {
        let hex_id = auto_bech32_to_hex(event_id)?;
        let hex_pk = auto_bech32_to_hex(event_pub_key)?;

        self.react_to(identity, &hex_id, &hex_pk, "+", difficulty_target)
    }

    #[cfg(feature = "async")]
    /// Add a like to an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_like() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    ///     client.like(&identity, "342060554ca30a9792f6e6959675ae734aed0223e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", 0).await.unwrap();
    /// }
    /// ```
    pub async fn like(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP25Error> {
        let hex_id = auto_bech32_to_hex(event_id)?;
        let hex_pk = auto_bech32_to_hex(event_pub_key)?;

        self.react_to(identity, &hex_id, &hex_pk, "+", difficulty_target)
            .await
    }

    #[cfg(not(feature = "async"))]
    /// Add a dislike to an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    /// let mut client = Client::new(vec![env!("RELAY_URL")]).unwrap();
    /// let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// client.dislike(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", 0).unwrap();
    /// ```
    pub fn dislike(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP25Error> {
        let hex_id = auto_bech32_to_hex(event_id)?;
        let hex_pk = auto_bech32_to_hex(event_pub_key)?;

        self.react_to(identity, &hex_id, &hex_pk, "-", difficulty_target)
    }

    #[cfg(feature = "async")]
    /// Add a dislike to an event
    ///
    /// # Example
    /// ```rust
    /// use nostr_rust::{nostr_client::Client, Identity};
    /// use std::str::FromStr;
    ///
    /// #[tokio::test]
    /// async fn test_dislike() {
    ///     let mut client = Client::new(vec![env!("RELAY_URL")]).await.unwrap();
    ///     let identity = Identity::from_str(env!("SECRET_KEY")).unwrap();
    /// client.dislike(&identity, "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6", 0).await.unwrap();
    /// }
    /// ```
    pub async fn dislike(
        &mut self,
        identity: &Identity,
        event_id: &str,
        event_pub_key: &str,
        difficulty_target: u16,
    ) -> Result<Event, NIP25Error> {
        let hex_id = auto_bech32_to_hex(event_id)?;
        let hex_pk = auto_bech32_to_hex(event_pub_key)?;

        self.react_to(identity, &hex_id, &hex_pk, "-", difficulty_target)
            .await
    }
}

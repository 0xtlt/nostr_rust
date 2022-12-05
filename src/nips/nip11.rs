use serde::{Deserialize, Serialize};
use thiserror::Error;

// Implementation of the NIP11 protocol
// https://github.com/nostr-protocol/nips/blob/master/11.md

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayInformationDocument {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub pubkey: Option<String>,
    pub contact: Option<String>,
    pub supported_nips: Option<Vec<u16>>,
    pub software: Option<String>,
    pub version: Option<String>,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NIP11Error {
    #[error("The relay information document is invalid")]
    InvalidRelayInformationDocument,

    #[error("The relay information document is not accessible")]
    RelayInformationDocumentNotAccessible,
}

#[cfg(not(feature = "async"))]
pub fn get_relay_information_document(
    relay_url: &str,
) -> Result<RelayInformationDocument, NIP11Error> {
    let relay_url = relay_url.replacen("ws", "http", 1);
    let relay_response: RelayInformationDocument = match reqwest::blocking::Client::new()
        .get(relay_url)
        .header("Accept", "application/nostr+json")
        .send()
    {
        Ok(response) => match response.json() {
            Ok(json) => json,
            Err(_) => return Err(NIP11Error::InvalidRelayInformationDocument),
        },
        Err(_) => return Err(NIP11Error::RelayInformationDocumentNotAccessible),
    };

    Ok(relay_response)
}

#[cfg(feature = "async")]
pub async fn get_relay_information_document(
    relay_url: &str,
) -> Result<RelayInformationDocument, NIP11Error> {
    let relay_url = relay_url.replacen("ws", "http", 1);
    let relay_response: RelayInformationDocument = match reqwest::Client::new()
        .get(relay_url)
        .header("Accept", "application/nostr+json")
        .send()
        .await
    {
        Ok(response) => match response.json().await {
            Ok(json) => json,
            Err(_) => return Err(NIP11Error::InvalidRelayInformationDocument),
        },
        Err(_) => return Err(NIP11Error::RelayInformationDocumentNotAccessible),
    };

    Ok(relay_response)
}

use crate::bech32::auto_bech32_to_hex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

// Implementation of the NIP5 protocol
// https://github.com/nostr-protocol/nips/blob/master/05.md
// nip05 is at the following format: username@domain

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NIP5Error {
    #[error("NIP05 must be at the format username@domain or _@domain")]
    InvalidFormat,

    #[error("NIP05 response is in an invalid format")]
    InvalidResponseFormat,

    #[error("Can't be accessed / Request failed")]
    RequestFailed,

    #[error("Public key doesn't match with the given NIP05 identifier")]
    MatchFailed,

    #[error("Bech32 Error: {}", _0)]
    Bech32Error(#[from] crate::bech32::Bech32Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrWellKnown {
    pub names: HashMap<String, String>,
}

#[cfg(not(feature = "async"))]
/// Check validity of a NIP05 identifier
///
/// # Example
/// ```rust
/// use nostr_rust::nips::nip5::{check_validity, NIP5Error};
///
/// assert_eq!(check_validity("_@nostr.0xtlt.dev", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6"), Ok(true));
/// assert_eq!(check_validity("_@nostr.0xtlt.dev", "3235036bd0957dfb27ccda02d452d7c763be40c91a1ac082ba6983b25238388c"), Ok(false));
/// assert_eq!(check_validity("_@", "3235036bd0957dfb27ccda02d452d7c763be40c91a1ac082ba6983b25238388c"), Err(NIP5Error::RequestFailed));
/// ```
pub fn check_validity(nip05: &str, pubkey: &str) -> Result<bool, NIP5Error> {
    let hex_pubkey = auto_bech32_to_hex(pubkey)?;
    let pubkey_found = get_nip05(nip05)?;

    Ok(pubkey_found == hex_pubkey)
}

#[cfg(feature = "async")]
/// Check validity of a NIP05 identifier
///
/// # Example
/// ```rust
/// use nostr_rust::nips::nip5::{check_validity, NIP5Error};
///
/// #[tokio::test]
/// async fn test_check_validity() {
///     assert_eq!(check_validity("_@nostr.0xtlt.dev", "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6").await, Ok(true));
///     assert_eq!(check_validity("_@nostr.0xtlt.dev", "3235036bd0957dfb27ccda02d452d7c763be40c91a1ac082ba6983b25238388c").await, Ok(false));
///     assert_eq!(check_validity("_@", "3235036bd0957dfb27ccda02d452d7c763be40c91a1ac082ba6983b25238388c").await, Err(NIP5Error::RequestFailed));
/// }
/// ```
pub async fn check_validity(nip05: &str, pubkey: &str) -> Result<bool, NIP5Error> {
    let hex_pubkey = auto_bech32_to_hex(pubkey)?;
    let pubkey_found = get_nip05(nip05).await?;

    Ok(pubkey_found == hex_pubkey)
}

#[cfg(not(feature = "async"))]
/// Get NIP05 Nostr Well Known of a domain
///
/// # Example
/// ```rust
/// use nostr_rust::nips::nip5::{get_nips05, NIP5Error};
///
/// assert_eq!(get_nips05("nostr.0xtlt.dev").is_ok(), true);
/// assert_eq!(get_nips05("nostr.0xtlt.dev").unwrap().names.get("_").unwrap(), "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6");
/// ```
pub fn get_nips05(domain: &str) -> Result<NostrWellKnown, NIP5Error> {
    // Check the domain
    let relay_response: NostrWellKnown = match reqwest::blocking::Client::new()
        .get(format!("https://{domain}/.well-known/nostr.json"))
        .send()
    {
        Ok(response) => match response.json() {
            Ok(json) => json,
            Err(_) => return Err(NIP5Error::InvalidResponseFormat),
        },
        Err(_) => return Err(NIP5Error::RequestFailed),
    };

    Ok(relay_response)
}

#[cfg(feature = "async")]
/// Get NIP05 Nostr Well Known of a domain
///
/// # Example
/// ```rust
/// use nostr_rust::nips::nip5::{get_nips05, NIP5Error};
///
/// #[tokio::test]
/// async fn test_get_nips05() {
///     assert_eq!(get_nips05("nostr.0xtlt.dev").await.is_ok(), true);
///     assert_eq!(get_nips05("nostr.0xtlt.dev").await.unwrap().names.get("_").unwrap(), "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6");
/// }
/// ```
pub async fn get_nips05(domain: &str) -> Result<NostrWellKnown, NIP5Error> {
    // Check the domain
    let relay_response: NostrWellKnown = match reqwest::Client::new()
        .get(format!("https://{domain}/.well-known/nostr.json"))
        .send()
        .await
    {
        Ok(response) => match response.json().await {
            Ok(json) => json,
            Err(_) => return Err(NIP5Error::InvalidResponseFormat),
        },
        Err(_) => return Err(NIP5Error::RequestFailed),
    };

    Ok(relay_response)
}

#[cfg(not(feature = "async"))]
/// Get the public key of a NIP05 identifier
///
/// # Example
/// ```rust
/// use nostr_rust::nips::nip5::{get_nip05, NIP5Error};
///
/// assert_eq!(get_nip05("_@nostr.0xtlt.dev"), Ok("884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string()));
/// ```
pub fn get_nip05(nip05: &str) -> Result<String, NIP5Error> {
    let parts: Vec<&str> = nip05.split('@').collect();

    // Check ["username", "domain"] length = 2
    if parts.len() != 2 {
        return Err(NIP5Error::InvalidFormat);
    }

    let list = get_nips05(parts[1])?;

    let pubkey = list.names.get(parts[0]);

    if let Some(pubkey) = pubkey {
        Ok(pubkey.clone())
    } else {
        Err(NIP5Error::MatchFailed)
    }
}

#[cfg(feature = "async")]
/// Get the public key of a NIP05 identifier
///
/// # Example
/// ```rust
/// use nostr_rust::nips::nip5::{get_nip05, NIP5Error};
///
/// #[tokio::test]
/// async fn test_get_nip05() {
///     assert_eq!(get_nip05("_@nostr.0xtlt.dev").await, Ok("884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string()));
/// }
/// ```
pub async fn get_nip05(nip05: &str) -> Result<String, NIP5Error> {
    let parts: Vec<&str> = nip05.split('@').collect();

    // Check ["username", "domain"] length = 2
    if parts.len() != 2 {
        return Err(NIP5Error::InvalidFormat);
    }

    let list = get_nips05(parts[1]).await?;

    let pubkey = list.names.get(parts[0]);

    if let Some(pubkey) = pubkey {
        Ok(pubkey.clone())
    } else {
        Err(NIP5Error::MatchFailed)
    }
}

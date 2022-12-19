use std::collections::HashMap;

use serde::{Deserialize, Serialize};
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrWellKnown {
    pub names: HashMap<String, String>,
}

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
    let pubkey_found = get_nip05(nip05)?;

    Ok(pubkey_found == pubkey)
}

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

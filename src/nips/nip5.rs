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

use crate::nostr_client::ClientError;
use thiserror::Error;

use super::nip5::NIP5Error;

// Implementation of the NIP1 protocol
// https://github.com/nostr-protocol/nips/blob/master/01.md

#[derive(Error, Debug, Eq, PartialEq)]
pub enum NIP1Error {
    #[error("No metadata given")]
    NoMetadata,

    #[error("The client has an error")]
    ClientError(ClientError),

    #[error("NIP05 error")]
    NIP05Error(NIP5Error),

    #[error("Given NIP05 is invalid with the given pubkey")]
    BadNIP05,
}

impl From<ClientError> for NIP1Error {
    fn from(err: ClientError) -> Self {
        Self::ClientError(err)
    }
}

impl From<NIP5Error> for NIP1Error {
    fn from(err: NIP5Error) -> Self {
        Self::NIP05Error(err)
    }
}

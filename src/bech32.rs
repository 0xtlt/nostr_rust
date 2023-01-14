use bech32::{FromBase32, ToBase32};
use hex::FromHexError;
use thiserror::Error;

pub enum ToBech32Kind {
    SecretKey,
    PublicKey,
    Note,
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Bech32Error {
    #[error("Invalid hex string")]
    InvalidHex,

    #[error("Bech32 given key is not a {0}")]
    InvalidKey(String),
}

impl From<bech32::Error> for Bech32Error {
    fn from(_err: bech32::Error) -> Self {
        Self::InvalidHex
    }
}

impl From<FromHexError> for Bech32Error {
    fn from(_err: hex::FromHexError) -> Self {
        Self::InvalidHex
    }
}

/// Transform a string (bech32 or hex) into an bech32 string
///
/// # Example
/// ```rust
/// use nostr_rust::bech32::{ToBech32Kind, to_bech32};
/// let bech32 = to_bech32(ToBech32Kind::PublicKey, "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d");
/// assert_eq!(bech32.unwrap(), "npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6");
/// ```
pub fn to_bech32(kind: ToBech32Kind, key: &str) -> Result<String, Bech32Error> {
    let key = key.to_string();

    match kind {
        ToBech32Kind::SecretKey => {
            if key.starts_with("nsec") {
                Ok(key)
            } else if key.starts_with("npub") || key.starts_with("note") {
                return Err(Bech32Error::InvalidKey("npub or note".to_string()));
            } else {
                return Ok(bech32::encode(
                    "nsec",
                    hex::decode(key)?.to_base32(),
                    bech32::Variant::Bech32,
                )?);
            }
        }
        ToBech32Kind::PublicKey => {
            if key.starts_with("npub") {
                Ok(key)
            } else if key.starts_with("nsec") || key.starts_with("note") {
                return Err(Bech32Error::InvalidKey("nsec or note".to_string()));
            } else {
                return Ok(bech32::encode(
                    "npub",
                    hex::decode(key)?.to_base32(),
                    bech32::Variant::Bech32,
                )?);
            }
        }
        ToBech32Kind::Note => {
            if key.starts_with("note") {
                Ok(key)
            } else if key.starts_with("nsec") || key.starts_with("npub") {
                return Err(Bech32Error::InvalidKey("nsec or npub".to_string()));
            } else {
                return Ok(bech32::encode(
                    "note",
                    hex::decode(key)?.to_base32(),
                    bech32::Variant::Bech32,
                )?);
            }
        }
    }
}

/// (hb for hex and bech32) Transform a string (bech32 or hex) into an hex string
/// # Example
/// ```rust
/// use nostr_rust::bech32::{ToBech32Kind, from_hb_to_hex};
/// let hex = from_hb_to_hex(ToBech32Kind::PublicKey, "npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6");
/// assert_eq!(hex.unwrap(), "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d");
/// ```
pub fn from_hb_to_hex(kind: ToBech32Kind, key: &str) -> Result<String, Bech32Error> {
    let key = key.to_string();

    match kind {
        ToBech32Kind::SecretKey => {
            if key.starts_with("nsec") {
                let data = bech32::decode(&key)?.1;
                let decoded = Vec::<u8>::from_base32(&data)?;
                let hex_str = hex::encode(decoded);

                Ok(hex_str)
            } else if key.starts_with("npub") {
                return Err(Bech32Error::InvalidKey("npub".to_string()));
            } else {
                return Ok(key);
            }
        }
        ToBech32Kind::PublicKey => {
            if key.starts_with("npub") {
                let data = bech32::decode(&key)?.1;
                let decoded = Vec::<u8>::from_base32(&data)?;
                let hex_str = hex::encode(decoded);

                Ok(hex_str)
            } else if key.starts_with("nsec") {
                return Err(Bech32Error::InvalidKey("nsec".to_string()));
            } else {
                return Ok(key);
            }
        }
        ToBech32Kind::Note => {
            if key.starts_with("note") {
                let data = bech32::decode(&key)?.1;
                let decoded = Vec::<u8>::from_base32(&data)?;
                let hex_str = hex::encode(decoded);

                Ok(hex_str)
            } else if key.starts_with("nsec") || key.starts_with("npub") {
                return Err(Bech32Error::InvalidKey("nsec or npub".to_string()));
            } else {
                return Ok(key);
            }
        }
    }
}

/// Transform a string (bech32 or hex) into an hex string
pub fn auto_bech32_to_hex(key: &str) -> Result<String, Bech32Error> {
    let key = key.to_string();

    if key.starts_with("nsec") {
        from_hb_to_hex(ToBech32Kind::SecretKey, &key)
    } else if key.starts_with("npub") {
        from_hb_to_hex(ToBech32Kind::PublicKey, &key)
    } else if key.starts_with("note") {
        from_hb_to_hex(ToBech32Kind::Note, &key)
    } else {
        Ok(key)
    }
}

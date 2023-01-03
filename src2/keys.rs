use rand::rngs::OsRng;
use secp256k1::{PublicKey, SecretKey, SECP256K1};

/// Get a random secret key
/// # Example
/// ```
/// use nostr_rust::keys::get_random_secret_key;
/// let (secret_key, public_key) = get_random_secret_key();
/// ```
pub fn get_random_secret_key() -> (SecretKey, PublicKey) {
    SECP256K1.generate_keypair(&mut OsRng)
}

/// Get a secret key from a hex string
/// # Example
/// ```rust
/// use nostr_rust::keys::secret_key_from_str;
/// let secret_key = secret_key_from_str(env!("SECRET_KEY"));
/// assert!(secret_key.is_ok());
/// ```
pub fn secret_key_from_str(s: &str) -> Result<SecretKey, String> {
    let decoded_hex = &hex::decode(s);
    match decoded_hex {
        Ok(decoded_hex) => match SecretKey::from_slice(decoded_hex) {
            Ok(secret_key) => Ok(secret_key),
            Err(_) => Err("Invalid secret key".to_string()),
        },
        Err(_) => Err("Invalid hex format".to_string()),
    }
}

/// Get a public key from a secret key
/// # Example
/// ```rust
/// use nostr_rust::keys::{secret_key_from_str, get_public_key_from_secret};
///
/// let secret_key = secret_key_from_str(env!("SECRET_KEY")).unwrap();
/// let public_key = get_public_key_from_secret(&secret_key);
/// ```
pub fn get_public_key_from_secret(secret_key: &SecretKey) -> PublicKey {
    PublicKey::from_secret_key(SECP256K1, secret_key)
}

/// Generate a hex secret key and a hex public key from a secret key
/// # Example
/// ```rust
/// use nostr_rust::keys::{secret_key_from_str, get_str_keys_from_secret};
///
/// let secret_key = secret_key_from_str(env!("SECRET_KEY")).unwrap();
/// let (secret_key_str, public_key_str) = get_str_keys_from_secret(&secret_key);
///
/// assert_eq!(secret_key_str, env!("SECRET_KEY"));
/// assert_eq!(public_key_str, env!("PUBLIC_KEY"));
/// ```
pub fn get_str_keys_from_secret(secret_key: &SecretKey) -> (String, String) {
    (
        secret_key.display_secret().to_string(),
        // Remove the 2 first characters because they are "0X" and useless
        normalize_public_key(&get_public_key_from_secret(secret_key).to_string()),
    )
}

/// Normalize a public key
pub fn normalize_public_key(public_key: &str) -> String {
    public_key.to_string()[2..].to_string()
}

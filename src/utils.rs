use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// Get actual timestamp in seconds
/// # Example
/// ```rust
/// use nostr_rust::utils::get_timestamp;
///
/// let timestamp = get_timestamp();
/// assert!(timestamp > 0);
/// ```
pub fn get_timestamp() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    since_the_epoch.as_secs()
}

/// Random sha256 hash
/// # Example
/// ```rust
/// use nostr_rust::utils::random_hash;
/// let hash = random_hash();
/// assert_eq!(hash.len(), 64);
/// ```
pub fn random_hash() -> String {
    let mut rng = rand::thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    sha256::digest(&bytes)
}

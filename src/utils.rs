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

const NPUB_LEN: usize = 63;
const NOTE_LEN: usize = 63;

#[derive(Debug, Clone)]
pub struct ParsedTagsResult {
    pub content: String,
    pub tags: Vec<Vec<String>>,
}

/// Parse string to generate tags
///
/// # Arguments
/// * `text` - Text to parse
/// * `hashtag_alphabet` - Alphabet to use for hashtag detection, None for no hashtag detection, you can use nostr_rust::DEFAULT_HASHTAG
/// * `detect_note` - Detect note tag
/// * `detect_npub` - Detect npub tag
///
/// # Example
/// ```rust
/// use nostr_rust::utils::parse_content_tags;
/// let tags = parse_content_tags("hello #world", vec![], Some(nostr_rust::DEFAULT_HASHTAG), true, true);
/// assert_eq!(tags.content, "hello #world");
/// assert_eq!(tags.tags, vec![vec!["t", "world"]]);
/// ```
pub fn parse_content_tags(
    text: &str,
    tags: Vec<Vec<String>>,
    hashtag_alphabet: Option<&str>,
    detect_note: bool,
    detect_npub: bool,
) -> ParsedTagsResult {
    let mut contents: Vec<String> = Vec::new();
    let mut tags = tags;

    let parts = text.split(' ');

    for part in parts {
        if let Some(hash) = hashtag_alphabet {
            if part.starts_with('#') {
                let is_hash = !part
                    .strip_prefix('#')
                    .unwrap()
                    .chars()
                    .any(|c| !hash.contains(c));

                if is_hash {
                    let s = part.strip_prefix('#').unwrap().to_string();
                    tags.push(vec!["t".to_string(), s]);
                }

                contents.push(part.to_string());

                continue;
            }
        }

        if detect_note && part.to_lowercase().starts_with("@note") && part.len() == (NOTE_LEN + 1) {
            let hex = crate::bech32::from_hb_to_hex(
                crate::bech32::ToBech32Kind::Note,
                &part.strip_prefix('@').unwrap().to_lowercase(),
            );

            if hex.is_ok() {
                tags.push(vec!["e".to_string(), hex.unwrap()]);
                let last_index = tags.len() - 1;
                contents.push(format!("#[{last_index}]"));
            } else {
                contents.push(part.to_string());
            }

            continue;
        }

        if detect_npub && part.to_lowercase().starts_with("@npub") && part.len() == (NPUB_LEN + 1) {
            let hex = crate::bech32::from_hb_to_hex(
                crate::bech32::ToBech32Kind::PublicKey,
                &part.strip_prefix('@').unwrap().to_lowercase(),
            );

            if hex.is_ok() {
                tags.push(vec!["p".to_string(), hex.unwrap()]);
                let last_index = tags.len() - 1;
                contents.push(format!("#[{last_index}]"));
            } else {
                contents.push(part.to_string());
            }

            continue;
        }

        contents.push(part.to_string());
    }

    ParsedTagsResult {
        content: contents.join(" "),
        tags,
    }
}

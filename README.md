# nostr_rust

[![crates.io](https://img.shields.io/crates/v/nostr_rust.svg)](https://crates.io/crates/nostr_rust)
[![Documentation](https://docs.rs/nostr_rust/badge.svg)](https://docs.rs/nostr_rust)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/nostr_rust.svg)](./LICENSE.txt)
[![CI](https://github.com/0xtlt/nostr_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/0xtlt/nostr_rust/actions/workflows/ci.yml)
[![Issues](https://img.shields.io/github/issues/0xtlt/nostr_rust)](https://img.shields.io/github/issues/0xtlt/nostr_rust)

An ergonomic, Nostr API Client for Rust.

- [Changelog](CHANGELOG.md)

## Example

This example uses [Tungstenite](https://crates.io/crates/tungstenite) for event handling, so your `Cargo.toml` could look like this:

```toml
[dependencies]
nostr_rust = "0.2"
tungstenite = "0.17"
```

And then the code:

```rust,norun
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};
use tungstenite::Message;

use nostr_rust::{nostr_client::Client, req::ReqFilter, Identity};

fn handle_message(relay_url: &String, message: &Message) -> Result<(), String> {
    println!("Received message from {}: {:?}", relay_url, message);

    Ok(())
}

fn main() {
    let my_identity =
        Identity::from_str("your private key as hex string")
            .unwrap();

    let nostr_client = Arc::new(Mutex::new(
        Client::new(vec!["wss://relay.nostr.info"]).unwrap(),
    ));

    // Run a new thread to handle messages
    let nostr_clone = nostr_client.clone();
    let handle_thread = thread::spawn(move || {
        println!("Listening...");
        let events = nostr_clone.lock().unwrap().next_data().unwrap();

        for (relay_url, message) in events.iter() {
            handle_message(relay_url, message).unwrap();
        }
    });

    // Change metadata
    nostr_client
        .lock()
        .unwrap()
        .set_metadata(
            &my_identity,
            Some("Rust Nostr Client test account"),
            Some("Hello Nostr! #5"),
            None,
        )
        .unwrap();

    // Subscribe to my last text note
    let subscription_id = nostr_client
        .lock()
        .unwrap()
        .subscribe(
            vec![ReqFilter {
                ids: None,
                authors: Some(vec![
                    "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
                ]),
                kinds: None,
                e: None,
                p: None,
                since: None,
                until: None,
                limit: Some(1),
            }],
        )
        .unwrap();

    // Unsubscribe
    nostr_client
        .lock()
        .unwrap()
        .unsubscribe(&subscription_id)
        .unwrap();

    // Publish a text note
    nostr_client
        .lock()
        .unwrap()
        .publish_text_note(&my_identity, "Hello Nostr! :)", &[])
        .unwrap();

    // Wait for the thread to finish
    handle_thread.join().unwrap();
}
```

## NIPs Supported

| NIP | Supported | Client Version | Description                                                  |
| --- | --------- | -------------- | ------------------------------------------------------------ |
| 01  | ✅        | 0.1.0          | Basic protocol flow description                              |
| 02  | ✅        | 0.3.0          | Contact List and Petnames                                    |
| 03  | ❌        | Not supported  | OpenTimestamps Attestations for Events                       |
| 04  | ❌        | Not supported  | Encrypted Direct Message                                     |
| 05  | ❌        | Not supported  | Mapping Nostr keys to DNS-based internet identifiers         |
| 06  | ❌        | Not supported  | Basic key derivation from mnemonic seed phrase               |
| 07  | ❌        | Not supported  | window.nostr capability for web browsers                     |
| 08  | ❌        | Not supported  | Handling Mentions                                            |
| 09  | ❌        | Not supported  | Event Deletion                                               |
| 10  | ❌        | Not supported  | Conventions for clients' use of e and p tags in text events. |
| 11  | ❌        | Not supported  | Relay Information Document                                   |
| 12  | ❌        | Not supported  | Generic Tag Queries                                          |
| 13  | ❌        | Not supported  | Proof of Work                                                |
| 14  | ❌        | Not supported  | Subject tag in text events.                                  |
| 15  | ❌        | Not supported  | End of Stored Events Notice                                  |
| 16  | ❌        | Not supported  | Event Treatment                                              |
| 22  | ❌        | Not supported  | Event created_at Limits                                      |
| 25  | ❌        | Not supported  | Reactions                                                    |
| 28  | ❌        | Not supported  | Public Chat                                                  |

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

# nostr_rust

[![crates.io](https://img.shields.io/crates/v/nostr_rust.svg)](https://crates.io/crates/nostr_rust)
[![Documentation](https://docs.rs/nostr_rust/badge.svg)](https://docs.rs/nostr_rust)
[![MIT/Apache-2 licensed](https://img.shields.io/crates/l/nostr_rust.svg)](./LICENSE.txt)
[![CI](https://github.com/0xtlt/nostr_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/0xtlt/nostr_rust/actions/workflows/ci.yml)
[![Issues](https://img.shields.io/github/issues/0xtlt/nostr_rust)](https://img.shields.io/github/issues/0xtlt/nostr_rust)

An ergonomic, [Nostr](https://github.com/nostr-protocol/nostr) API Client for Rust.

- [Changelog](CHANGELOG.md)

## Example

```toml
[dependencies]
nostr_rust = "*"
```

And then the code:

```rust,norun
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
    thread,
};

use nostr_rust::{nostr_client::Client, req::ReqFilter, Identity, Message, events::extract_events_ws, utils::parse_content_tags};

fn handle_message(relay_url: &String, message: &Message) -> Result<(), String> {
    println!("Received message from {}: {:?}", relay_url, message);

    let events = extract_events_ws(message);
    println!("Events: {:?}", events);

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
            None,
            0,
        )
        .unwrap();

    // Subscribe to my last text note
    let subscription_id = nostr_client
        .lock()
        .unwrap()
        .subscribe(vec![ReqFilter {
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
        }])
        .unwrap();

    // Unsubscribe
    nostr_client
        .lock()
        .unwrap()
        .unsubscribe(&subscription_id)
        .unwrap();

    // You can use the parse content tags method to get the content and the tags from a string
    // let tags = parse_content_tags("hello #world", vec![], Some(nostr_rust::DEFAULT_HASHTAG), true, true);
    // assert_eq!(tags.content, "hello #world");
    //  assert_eq!(tags.tags, vec![vec!["t", "world"]]);

    // Publish a text note
    nostr_client
        .lock()
        .unwrap()
        .publish_text_note(&my_identity, "Hello Nostr! :)", &[], 0)
        .unwrap();

    // Publish a proof of work text note with a difficulty target of 15
    nostr_client
        .lock()
        .unwrap()
        .publish_text_note(&my_identity, "Hello Nostr! :)", &[], 15)
        .unwrap();

    // Wait for the thread to finish
    handle_thread.join().unwrap();
}

## Async feature

If you want to use the async version of the client, you can enable the `async` feature:

```toml
[dependencies]
nostr_rust = { version = "*", features = ["async"] }
```

## NIPs Supported

| NIP                                                            | Supported     | Client Version | Description                                                  |
| -------------------------------------------------------------- | ------------- | -------------- | ------------------------------------------------------------ |
| [01](https://github.com/nostr-protocol/nips/blob/master/01.md) | ✅            | 0.1.0          | Basic protocol flow description                              |
| [02](https://github.com/nostr-protocol/nips/blob/master/02.md) | ✅            | 0.3.0          | Contact List and Petnames                                    |
| [03](https://github.com/nostr-protocol/nips/blob/master/03.md) | ❌            | Not supported  | OpenTimestamps Attestations for Events                       |
| [04](https://github.com/nostr-protocol/nips/blob/master/04.md) | ✅            | 0.6.0          | Encrypted Direct Message                                     |
| [05](https://github.com/nostr-protocol/nips/blob/master/05.md) | ✅            | 0.15.0  | Mapping Nostr keys to DNS-based internet identifiers         |
| [06](https://github.com/nostr-protocol/nips/blob/master/06.md) | ❌            | Not supported  | Basic key derivation from mnemonic seed phrase               |
| [07](https://github.com/nostr-protocol/nips/blob/master/07.md) | Not concerned | Not supported  | window.nostr capability for web browsers                     |
| [08](https://github.com/nostr-protocol/nips/blob/master/08.md) | Not concerned            | Not supported  | Handling Mentions                                            |
| [09](https://github.com/nostr-protocol/nips/blob/master/09.md) | ✅            | 0.5.0          | Event Deletion                                               |
| [10](https://github.com/nostr-protocol/nips/blob/master/10.md) | Not concerned            | Not supported  | Conventions for clients' use of e and p tags in text events. |
| [11](https://github.com/nostr-protocol/nips/blob/master/11.md) | ✅            | 0.9.0          | Relay Information Document                                   |
| [12](https://github.com/nostr-protocol/nips/blob/master/12.md) | ❌            | Not supported  | Generic Tag Queries                                          |
| [13](https://github.com/nostr-protocol/nips/blob/master/13.md) | ✅            | 0.8.0          | Proof of Work                                                |
| [14](https://github.com/nostr-protocol/nips/blob/master/14.md) | Not concerned            | Not supported  | Subject tag in text events.                                  |
| [15](https://github.com/nostr-protocol/nips/blob/master/15.md) | ❌            | Not supported  | End of Stored Events Notice                                  |
| [16](https://github.com/nostr-protocol/nips/blob/master/16.md) | ✅            | 0.13.0         | Event Treatment                                              |
| [22](https://github.com/nostr-protocol/nips/blob/master/22.md) | ❌            | Not supported  | Event created_at Limits                                      |
| [25](https://github.com/nostr-protocol/nips/blob/master/25.md) | ✅            | 0.4.0          | Reactions                                                    |
| [28](https://github.com/nostr-protocol/nips/blob/master/28.md) | ❌            | Not supported  | Public Chat                                                  |

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

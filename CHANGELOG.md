# Changelog

## 0.20.3

- Fix: `nips::nip04::decrypt` method - Now working again  (@0xtlt)

## 0.20.2

- Fix: `Client::get_events_of` Waiting for all relays (@0xtlt)

## 0.20.1

- Edit: Everywhere where we need to give a public key hex or event id hex, we can now give a bech32 public key (npub and note) except for `Client::set_contact_list` method (@0xtlt)

## 0.20.0

- Add: `bech32::auto_bech32_to_hex` method - Auto detect bech32 (note, nsec or npub,) or hex and force convert to hex
- Fix: NIP04 encrypt and decrypt - Now working again

## 0.19.2

- Fix: `Client::get_events_of` method - Now skip non events messages

## 0.19.1

- Fix: `Client::get_events_of` method - Now skip non events messages

## 0.19.0

- Add: `Identity::make_event` method
- Add: `Client::broadcast_event` method

## 0.18.0 - Content

- Add: `utils::parse_content_tags` method
- Add: `utils::ParsedTagsResult` struct

## 0.17.0

- Add: `event::extract_events` method
- Add: `event::extract_events_ws` method

## 0.16.0 - Async support

- Add: `async` feature to enable async support and disable sync

## 0.15.0 - NIP 05 Support

- Add `nips::nip05::check_validity` method
- Add `nips::nip05::get_nip05` method
- Add `nips::nip05::get_nips05` method
- Add `nips::nip05::NostrWellKnown` structure
- Add `nips::nip05::NIP5Error` enum

## 0.14.0 - Event Verify

- Add: `Event.get_content` method
- Add: `Event.get_content_id` method
- Add: `Event.verify` method
- Add: `EventError` enum

## 0.13.1 - ReqFilter kinds to u16

- Edit: `kinds` will now be `Option<Vec<u16>>` instead of `Option<Vec<u8>>` in `ReqFilter`

## 0.13.0 - NIP-16 Support

- Add: `Client.publish_replaceable_event`
- Add: `Client.publish_ephemeral_event`
- Add: `nips::nip16:NIP16Error` enum
- Add: `nips::nip16::publish_nip16_event`

## 0.12.0 - More kinds of things

- Edit: `Event` `kind` field from u8 to u16
- Edit: `EventPrepare` `kind` field from u8 to u16

## 0.11.0 - Bech32 support

- Add: `bech32::to_bech32` method to convert a `Hex` to a bech32 address
- Add: `bech32::from_hb_to_hex` method to convert a bech32 address to a `Hex` (hb for hex and bech32)

## 0.10.0 - NIP-13 better Support

- Edit: `EventPrepare.to_event` method accepts pow difficulty target
- Edit: `EventPrepare::count_leading_zero_bits` method returns u16
- Edit: `EventPrepare.to_pow_event` method returns Result<(), NIP13Error>
- Edit: `Client.set_metadata` method accepts pow difficulty target
- Edit: `Client.publish_text_note` method accepts pow difficulty target
- Edit: `Client.add_recommended_relay` method accepts pow difficulty target
- Edit: `Client.set_contact_list` method accepts pow difficulty target
- Edit: `Client.react_to` method accepts pow difficulty target
- Edit: `Client.like` method accepts pow difficulty target
- Edit: `Client.dislike` method accepts pow difficulty target
- Edit: `Client.send_private_message` method accepts pow difficulty target
- Edit: `Client.delete_event` method accepts pow difficulty target
- Edit: `Client.delete_event_with_reason` method accepts pow difficulty target
- Remove: `Client.publish_pow_text_note`

## 0.9.0 - NIP-11 Support

- Add: `nips::nip11::get_relay_information_document` method

## 0.8.0 - NIP-13 Support

- Add: `nips::nip13::NIP13Error` enum
- Add: `EventPrepare::count_leading_zero_bits` method
- Add: `EventPrepare.to_pow_event` method
- Add: `Client.publish_pow_text_note` method

## 0.7.0 - Errors and SSL

- Edit: `OpenSSL` dependency replaced by `rustls`
- Edit: Errors are now enums
- Edit: Added `Message` type

## 0.6.0 - NIP-04 Support

- Add: NIP-04 support
- Add: `nips::nip4::PrivateMessage` structure
- Add: `nips::nip4::decrypt` method
- Add: `nips::nip4::encrypt` method
- Add: `Client.send_private_message` method
- Add: `Client.get_private_events_with` method
- Add: `Client.get_private_messages_with` method

## 0.5.0 - NIP-09 Support

- Edit: `Client.publish_text_note` method returns the Event object
- Edit: `Client.delete_event` method
- Edit: `Client.delete_event_with_reason` method

## 0.4.0 - NIP-25 Support

- Added: `Client.react_to` method
- Added: `Client.like` method
- Added: `Client.dislike` method

## 0.3.0 - NIP-02 Support

- Added: `Client.get_contact_list` method
- Added: `Client.set_contact_list` method
- Added: `Client.add_event` method
- Added: `Client.get_events` method
- Added: `Client.get_events_of` method
- Added: `ContactListTag` structure
- Added: `ContactListTag.to_tags` method

## 0.2.0 - Architecture change

- Removed: `Client.listen` function (Replaced by `Client.next_data`)
- Added: `Client.next_data` function
- Added: `Client.remove_relay` function

## 0.1.0 - NIP-01 Support

- Added: `Client` structure
- Added: `Client::new` function
- Added: `Client.add_relay` function
- Added: `Client.listen` function
- Added: `Client.subscribe` function
- Added: `Client.subscribe_with_id` function
- Added: `Client.unsubscribe` function
- Added: `Identity` structure
- Added: `Identity::from_str` function with hex private key support
- Added: `random_hash` function
- Added: `get_timestamp` function
- Added: `get_random_secret_key` function
- Added: `secret_key_from_str` function
- Added: `get_public_key_from_secret` function
- Added: `get_str_keys_from_secret` function
- Added: `EventPrepare` structure
- Added: `EventPrepare.get_content` function
- Added: `EventPrepare.get_content_id` function
- Added: `EventPrepare.to_event` function
- Added: `Event` structure
- Added: `Event.to_string` function
- Added `Req` structure
- Added: `Req::new` function
- Added: `Req.get_close_event` function
- Added: `Req.to_string()` function
- Added: `ReqFilter` structure
- Added: `ReqFilter.to_json` function
- Added: `SimplifiedWS` structure
- Added: `SimplifiedWS::new` function
- Added: `SimplifiedWS.send_message` function
- Added: `SimplifiedWS.read_message` function

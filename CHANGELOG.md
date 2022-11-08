# Changelog

## 0.6.0 - NIP-04 Support

- Add: NIP-04 support
- Add: `nips::nip4::decrypt` method
- Add: `nips::nip4::encrypt` method
- Add: `Client.send_private_message` method

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

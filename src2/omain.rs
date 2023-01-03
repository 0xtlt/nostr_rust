// use bech32::{FromBase32, ToBase32, Variant};
use nostr_rust::{
    bech32::{to_bech32, ToBech32Kind},
    nostr_client::Client,
};

// p: 1c64d7be2a1d4a54a03648115b3cdb349083c7e714d5d41d187cffb4621cdc26
// pub: 7291e796d195ce1addcd5cf3c431fa3004aff5203d4986fd6799e45b7bbfd874

// fn _handle_message(relay_url: String, _message: Message) -> Result<(), String> {
//     println!("Received message from {}", { relay_url });

//     Ok(())
// }

fn main() {
    // let nostr_client = Client::new(vec!["ws://localhost:8080"]).unwrap();

    // let my_identity =
    //     Identity::from_str("1c64d7be2a1d4a54a03648115b3cdb349083c7e714d5d41d187cffb4621cdc26")
    //         .unwrap();

    // npub13prsf02zzus79yhdhl6zadm4gllpzhr0lxp9kz8uxe47fntfa8mqyrh6a9
    // 884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6

    // let pk = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    // let b32 = bech32::encode("npub", pk.to_string().to_base32(), bech32::Variant::Bech32).unwrap();
    // let bech32 = to_bech32(
    //     ToBech32Kind::PublicKey,
    //     "3bf0c63fcb93463407af97a5e5ee64fa883d107ef9e558472c4eb9aaaefa459d",
    // );
    // assert_eq!(
    //     bech32.unwrap(),
    //     "npub180cvv07tjdrrgpa0j7j7tmnyl2yr6yr7l8j4s3evf6u64th6gkwsyjh6w6"
    // );

    // let public_key_hex = "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6";
    // let public_key_bytes = hex::decode(public_key_hex).unwrap();

    // let encoded = bech32::encode(
    //     "npub",
    //     // vec![0x00, 0x01, 0x02].to_base32(),
    //     public_key_bytes.to_base32(),
    //     Variant::Bech32,
    // )
    // .unwrap();

    // let encoded = "npub13prsf02zzus79yhdhl6zadm4gllpzhr0lxp9kz8uxe47fntfa8mqyrh6a9";

    // println!("Encoded: {}", encoded);
    // let (hrp, data, variant) = bech32::decode(&encoded).unwrap();
    // let decoded = Vec::<u8>::from_base32(&data).unwrap();
    // let hex_str_2 = hex::encode(decoded);
    // println!("Decoded: {:?}", Vec::<u8>::from_base32(&data).unwrap());
    // println!("Decoded: {:?}", hex_str_2);

    // println!("b32: {}", b32);

    // println!("my_identity: {:?}", my_identity.get_public_key_as_bech32());
    // println!("my_identity: {:?}", my_identity.public_key_str);
    // let nostr_client = Arc::new(Mutex::new(
    //     Client::new(vec!["wss://relay.damus.io", "wss://relay.nostr.info"]).unwrap(),
    // ));

    // get_relay_information_document("https://relay.damus.io");

    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .publish_pow_text_note(&my_identity, "PoW 25", &[], 25)
    //     .unwrap();

    // let mut event = EventPrepare {
    //     pub_key: my_identity.public_key_str.clone(),
    //     created_at: 0,
    //     kind: 0,
    //     tags: vec![],
    //     content: "content".to_string(),
    // };

    // let difficulty = 20;
    // let nostr_event = event.to_pow_event(&my_identity, difficulty).unwrap();
    // let event_id = hex::decode(nostr_event.id).unwrap();
    // let event_difficulty = EventPrepare::count_leading_zero_bits(event_id);
    // println!("Event difficulty: {}", event_difficulty);
    // assert!(event_difficulty >= difficulty);
    // assert_eq!(nostr_event.content, "content");
    // assert_eq!(nostr_event.kind, 0);
    // assert_eq!(nostr_event.tags.len(), 1);
    // assert_eq!(nostr_event.created_at, 0);
    // assert_eq!(nostr_event.pub_key, my_identity.public_key_str);
    // assert_eq!(nostr_event.sig.len(), 128);

    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .send_private_message(
    //         &my_identity,
    //         "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6",
    //         "Hello world! :)",
    //     )
    //     .unwrap();

    // println!(
    //     "Connecting to Nostr..., {:?}",
    //     nostr_client.lock().unwrap().get_private_messages_with(
    //         &my_identity,
    //         "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6",
    //         1
    //     )
    // );

    // 02 for public key
    // const MSG: &str = "helloworld";
    // let (sk, pk) = generate_keypair();
    // let (sk, pk) = (&sk.serialize(), &pk.serialize());
    // let msg = MSG.as_bytes();
    // let en = encrypt(pk, msg).unwrap();
    // let de = decrypt(sk, &en)
    //     .unwrap()
    //     .iter()
    //     .map(|&x| x as char)
    //     .collect::<String>();

    // println!("en: {:?}", en);
    // println!("de: {:?}", de);

    // let clement_pub =
    //     hex::decode("025043db606d36e48adb85b129d80729ff89e84496e4575ae6a6b0631f078826c5").unwrap();

    // let message = ecies::encrypt(&clement_pub, "hello world".as_bytes());
    // println!("message: {:?} - {}", message, clement_pub.len());

    // let m = ecies::utils::aes_encrypt(&thomas.secret_key.secret_bytes(), msg).unwrap();
    // println!("m: {:?}", m);
    // let p = ecies::utils::aes_decrypt(&thomas.secret_key.secret_bytes(), &m)
    //     .unwrap()
    //     .iter()
    //     .map(|&x| x as char)
    //     .collect::<String>();
    // println!("p: {:?}", p);

    // assert_eq!(
    //     msg,
    //     decrypt(sk, &encrypt(pk, msg).unwrap()).unwrap().as_slice()
    // );

    // let pub_key = "02884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".as_bytes();
    // let msg: &str = "Jk8AKIgKsXUD0jdgPnoj2g==?iv=wKt+WglCth2/qp5zQATRUQ==";
    // let c = encrypt(pub_key, "helloworld".as_bytes());
    // println!("{:?}", c);
    // let c = decrypt(&thomas.secret_key.secret_bytes(), msg.as_bytes());
    // println!("Decrypted: {:?}", c);

    // println!("decrypt={:?}", decrypt(&hex_key, msg, iv));

    // let handle_thread = thread::spawn(move || loop {
    //     let response = nostr_clone.lock().unwrap().next_data();
    //     println!("Received message from {:?}", { response });
    // });

    // Change metadata
    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .set_metadata(
    //         &my_identity,
    //         Some("Rust Nostr Client test account"),
    //         Some("Hello Nostr! #5"),
    //         None,
    //     )
    //     .unwrap();

    // Subscribe to my last text note

    // println!(
    //     "mmmm={:?}",
    //     nostr_client
    //         .lock()
    //         .unwrap()
    //         .get_events_by_author(
    //             "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6"
    //         )
    //         .unwrap()
    // );
    // println!(
    //     "contact:{:?}",
    //     nostr_client
    //         .lock()
    //         .unwrap()
    //         .get_contact_list("884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6")
    //         .unwrap()
    // );

    // let event = nostr_client
    //     .lock()
    //     .unwrap()
    //     .publish_text_note(&my_identity, "Hello Nostr! #6", &[])
    //     .unwrap();
    // let event = nostr_client
    //     .lock()
    //     .unwrap()
    //     .delete_event(
    //         &my_identity,
    //         "1806fff51b01ef5f33842adf33ee65819940d246e39a38539eb5ce3a8d06bad5",
    //     )
    //     .unwrap();

    // println!("event={:?}", event);

    // event=Event { id: "1806fff51b01ef5f33842adf33ee65819940d246e39a38539eb5ce3a8d06bad5", pub_key: "7291e796d195ce1addcd5cf3c431fa3004aff5203d4986fd6799e45b7bbfd874", created_at: 1667771475, kind: 1, tags: [], content: "Hello Nostr! #6", sig: "a930faac1b58c458223a73630edb71a28805324fcfeddd409560feddba53f17b8869c90a691b34e9148a2386566ab0cba04f086429ae6f5f16c080bc294085cc" }

    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .remove_relay("wss://relay.nostr.info")
    //     .unwrap();

    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .subscribe(vec![ReqFilter {
    //         ids: None,
    //         authors: Some(vec![
    //             "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    //         ]),
    //         kinds: None,
    //         e: None,
    //         p: None,
    //         since: None,
    //         until: None,
    //         limit: Some(1),
    //     }])
    //     .unwrap();

    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .set_contact_list(
    //         &my_identity,
    //         vec![ContactListTag {
    //             key: "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6".to_string(),
    //             main_relay: Some("wss://nostr-pub.wellorder.net".to_string()),
    //             surname: Some("Rust Nostr Client".to_string()),
    //         }],
    //     )
    //     .unwrap();

    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .react_to(
    //         &my_identity,
    //         "342060554ca30a9792f6e6959675ae734aed02c23e35037d2a0f72ac6316e83d",
    //         "884704bd421721e292edbff42eb77547fe115c6ff9825b08fc366be4cd69e9f6",
    //         "✅",
    //     )
    //     .unwrap();

    // Wait 5s and remove the relay
    // thread::sleep(std::time::Duration::from_secs(2));

    // // Unsubscribe
    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .unsubscribe(&subscription_id)
    //     .unwrap();

    // // Publish a text note
    // nostr_client
    //     .lock()
    //     .unwrap()
    //     .publish_text_note(&my_identity, "Hello Nostr! #5", &[])
    //     .unwrap();

    // Wait for the thread to finish
    // handle_thread.join().unwrap();

    // wait 10s
    // thread::sleep(std::time::Duration::from_secs(60));
    // handle_thread.join().unwrap();

    // let event = EventPrepare {
    //     pub_key: env!("PUBLIC_KEY").to_string(),
    //     created_at: 0, // Don't use this in production
    //     kind: 0,
    //     tags: vec![],
    //     content: "content".to_string(),
    // };

    // let identity =
    //     Identity::from_str(env!("SECRET_KEY"))
    //         .unwrap();
    // let nostr_event = event.to_event(&identity);

    // let m = nostr_event.to_string();

    // println!("m = {}", m);

    // client.add_relay("wss://nostr-pub.wellorder.net");

    // let m = get_keys(&l);

    // println!("{:?}", m);

    // println!(
    //     "{:?}",
    //     client.set_metadata(
    //         (&l, &m.1),
    //         Some("Rust Nostr Client test account"),
    //         Some("it's me Mario"),
    //         None
    //     )
    // );
}

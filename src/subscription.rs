use crate::{
    events::Event,
    nostr_client::{Client, ClientError},
    req::ReqFilter,
};
use std::collections::HashMap;

pub enum SubscriptionMessage {
    /// ["EVENT", <subscription_id>, <event JSON as defined above>]
    Event(String, Event),
    /// ["NOTICE", <message>]
    Notice(String),
    /// ["OK", <event_id>, <true|false>, <message>]
    Ok(String, bool, String),
}

impl Client {
    pub async fn listen(&self) -> Result<(), ClientError> {
        loop {
            let events = self.next_data().await?;

            for (relay_url, message) in events.iter() {
                if let tungstenite::Message::Text(text) = message {
                    let json = serde_json::from_str::<Vec<serde_json::Value>>(text);

                    if json.is_err() {
                        continue;
                    }

                    let json = json.unwrap();

                    if json.is_empty() {
                        continue;
                    }

                    let relay_message_type = &json[0].as_str();

                    if relay_message_type.is_none() {
                        continue;
                    }

                    let relay_message_type = relay_message_type.unwrap();

                    match relay_message_type {
                        "EVENT" => {
                            let events = crate::events::extract_events(text);

                            let subscription_id = {
                                if json.len() < 2 {
                                    continue;
                                }

                                if let Some(subscription_id) = json[1].as_str() {
                                    if !subscription_id.starts_with('{') {
                                        Some(subscription_id.to_string())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            };

                            if subscription_id.is_none() {
                                continue;
                            }

                            let subscription_id = subscription_id.unwrap();

                            let mut subscription_obj = self.subscription_pool.lock().await;

                            let subscription_obj =
                                subscription_obj.subscriptions.get_mut(&subscription_id);

                            if subscription_obj.is_none() {
                                continue;
                            }

                            let subscription_obj = subscription_obj.unwrap();

                            // TODO: for me, continue here
                        }
                        "OK" => {}
                        _ => {
                            continue;
                        }
                    };
                }
            }
        }
    }
}

pub struct SubscriptionPool {
    // will panic if is listening is false
    is_listening: bool,
    subscriptions: HashMap<String, (Vec<ReqFilter>, Vec<SubscriptionMessage>)>,
}

impl SubscriptionPool {
    pub fn new() -> Self {
        Self {
            is_listening: false,
            subscriptions: HashMap::new(),
        }
    }

    pub fn listen(&mut self) {
        self.is_listening = true;
    }

    pub fn stop(&mut self) {
        self.is_listening = false;
    }

    pub fn is_listening(&self) -> bool {
        self.is_listening
    }
}

impl Default for SubscriptionPool {
    fn default() -> Self {
        Self::new()
    }
}

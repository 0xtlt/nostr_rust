use crate::utils::random_hash;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

/// Req struct is used to request events and subscribe to new updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Req {
    /// `<subscription_id>` is a random string that should be used to represent a subscription.
    pub subscription_id: String,
    /// `<filters>` is a JSON object that determines what events will be sent in that subscription, it can have the following attributes:
    pub filters: Vec<ReqFilter>,
}

/// ReqFilter is a JSON object that determines what events will be sent in that subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReqFilter {
    /// a list of event ids or prefixes
    pub ids: Option<Vec<String>>,
    /// a list of pubkeys or prefixes, the pubkey of an event must be one of these
    pub authors: Option<Vec<String>>,
    /// a list of a kind numbers
    pub kinds: Option<Vec<u16>>,
    /// a list of event ids that are referenced in an "e" tag
    #[serde(rename = "#e")]
    pub e: Option<Vec<String>>,
    /// a list of pubkeys that are referenced in a "p" tag
    #[serde(rename = "#p")]
    pub p: Option<Vec<String>>,
    /// a timestamp, events must be newer than this to pass
    pub since: Option<u64>,
    /// a timestamp, events must be older than this to pass
    pub until: Option<u64>,
    /// maximum number of events to be returned in the initial query
    pub limit: Option<u64>,
}

impl ReqFilter {
    /// Return a clean json object (Value)
    pub fn to_json(&self) -> serde_json::Value {
        let mut json = json!({});

        if let Some(ids) = &self.ids {
            json["ids"] = json!(ids);
        }

        if let Some(authors) = &self.authors {
            json["authors"] = json!(authors);
        }

        if let Some(kinds) = &self.kinds {
            json["kinds"] = json!(kinds);
        }

        if let Some(e) = &self.e {
            json["#e"] = json!(e);
        }

        if let Some(p) = &self.p {
            json["#p"] = json!(p);
        }

        if let Some(since) = &self.since {
            json["since"] = json!(since);
        }

        if let Some(until) = &self.until {
            json["until"] = json!(until);
        }

        if let Some(limit) = &self.limit {
            json["limit"] = json!(limit);
        }

        json
    }
}

impl Req {
    pub fn new(subscription_id: Option<&str>, filters: Vec<ReqFilter>) -> Self {
        Self {
            subscription_id: subscription_id.unwrap_or(&random_hash()).to_string(),
            filters,
        }
    }

    pub fn get_close_event(&self) -> String {
        json!({
            "subscription_id": self.subscription_id,
            "close": true
        })
        .to_string()
    }
}

impl fmt::Display for Req {
    /// Return the serialized event
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut req = json!(["REQ", self.subscription_id]);
        for filter in &self.filters {
            req.as_array_mut().unwrap().push(filter.to_json());
        }

        write!(f, "{}", serde_json::to_string(&req).unwrap())
    }
}

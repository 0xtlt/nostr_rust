use crate::events::Event;

pub enum SubscriptionMessage {
    Event(Event),
}

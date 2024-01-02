use serde::de::DeserializeOwned;

use super::Change;

pub struct Store<T: DeserializeOwned> {
    topic: String,
    items: Vec<T>,
}

impl<T: DeserializeOwned> Store<T> {
    pub fn new(topic: String) -> Self {
        Self {
            topic,
            items: Vec::new(),
        }
    }
}

pub trait Subscriber {
    fn notify(&mut self, topic: &str, change: &Change);
}

impl<T: super::change::Record + DeserializeOwned> Subscriber for Store<T> {
    fn notify(&mut self, change_topic: &str, change: &Change) {
        if self.topic == change_topic {
            change.apply(&mut self.items)
        }
    }
}

use std::{cell::{RefCell, Ref}, rc::Rc};

use serde::de::DeserializeOwned;

use super::Change;

pub struct Store<T: DeserializeOwned> {
    topic: String,
    items: Rc<RefCell<Vec<T>>>,
}

impl<T: DeserializeOwned> Store<T> {
    pub fn new(topic: String) -> Self {
        Self {
            topic,
            items: Rc::new(RefCell::new(Vec::new())),
        }
    }

    pub fn read(&self) -> Ref<'_, Vec<T>> {
        self.items.borrow()
    }

    pub fn update<O>(&self, mutable_callback: impl FnOnce(&mut Vec<T>) -> O) -> O {
        mutable_callback(&mut *self.items.borrow_mut())
    }
}

pub trait Subscriber {
    fn notify(&self, topic: &str, change: &Change);
}

impl<T: super::change::Record + DeserializeOwned> Subscriber for Store<T> {
    fn notify(&self, change_topic: &str, change: &Change) {
        if self.topic == change_topic {
            change.apply(&mut self.items.borrow_mut())
        }
    }
}

use ::dioxus::prelude::*;
use ::gloo_storage::{LocalStorage, Storage};
use ::serde::{de::DeserializeOwned, Serialize};

#[allow(clippy::needless_return)]
pub fn use_persistent<T: Serialize + DeserializeOwned + Default + 'static>(
    key: impl ToString,
    init: impl FnOnce() -> T,
) -> UsePersistent<T> {
    let state = use_signal(move || {
        let key = key.to_string();
        let value = LocalStorage::get(key.as_str()).ok().unwrap_or_else(init);
        StorageEntry { key, value }
    });

    UsePersistent { inner: state }
}

struct StorageEntry<T> {
    key: String,
    value: T,
}

pub struct UsePersistent<T: 'static> {
    inner: Signal<StorageEntry<T>>,
}

impl<T> Clone for UsePersistent<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for UsePersistent<T> {}

impl<T: Serialize + DeserializeOwned + Clone + 'static> UsePersistent<T> {
    pub fn get(&self) -> T {
        self.inner.read().value.clone()
    }

    pub fn set(&mut self, value: T) {
        let mut inner = self.inner.write();
        let _ = LocalStorage::set(inner.key.as_str(), &value);
        inner.value = value;
    }
}
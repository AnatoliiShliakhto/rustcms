use ::dioxus::prelude::FormEvent;

pub trait FormEventExt {
    fn value(&self, key: &str) -> Option<String>;
    fn values(self, key: &str) -> Option<Vec<String>>;
}

impl FormEventExt for FormEvent {
    fn value(&self, key: &str) -> Option<String> {
        self.data.values().get(key).map(|value| value[0].clone())
    }

    fn values(self, key: &str) -> Option<Vec<String>> {
        self.data.values().get(key).map(|value| value.0.clone())
    }
}
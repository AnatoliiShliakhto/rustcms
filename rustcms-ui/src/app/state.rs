use ::dioxus::prelude::*;
use ::js_sys::{Reflect::get, wasm_bindgen::JsValue};
use ::reqwest::{Client, Url};
use ::web_sys::window;

use crate::components::DialogArgs;

#[derive(Clone, Copy)]
pub struct UseState {
    pub client: Signal<Client>,
    host: Signal<Url>,
    pub dialog_args: Signal<Option<DialogArgs>>,
}

impl UseState {
    pub fn client(&self) -> Client {
        self.client.read().clone()
    }

    pub fn url(&self, path: &str) -> Url {
        self.host
            .read()
            .clone()
            .join(path)
            .unwrap_or(self.host.read().clone())
    }
}

pub fn use_init_state() -> UseState {
    use_context_provider(|| UseState {
        client: Signal::new(Client::new()),
        host: Signal::new(location()),
        dialog_args: Signal::new(None),
    })
}

pub fn use_state() -> UseState {
    consume_context::<UseState>()
}

fn location() -> Url {
    if let Ok(host) = get(
        &JsValue::from(window().unwrap().location()),
        &JsValue::from_str("origin"),
    ) {
        host.as_string().unwrap().as_str().parse().unwrap()
    } else {
        "https://localhost".parse().unwrap()
    }
}

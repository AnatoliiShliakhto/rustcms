#![forbid(unsafe_code)]
#![allow(non_snake_case)]

mod app;
mod components;
mod elements;
mod macros;
mod pages;
mod services;

use ::dioxus::{
    document::eval,
    logger::{
        init as logger_init,
        tracing::{Level, info},
    },
    prelude::*,
};
use ::dioxus_i18n::{prelude::*, unic_langid::langid};
use ::serde_json::Value;

use crate::app::{Route, use_init_state};

pub static LANGUAGES: [&str; 2] = ["en-US", "uk-UA"];
static THEMES: [&str; 5] = ["default", "light", "dark", "emerald", "dracula"];

fn main() {
    logger_init(Level::INFO).expect("failed to init logger");
    info!("RustCMS UI v{}", env!("CARGO_PKG_VERSION"));

    launch(|| {
        use_init_state();

        use_init_i18n(|| {
            I18nConfig::new(langid!("en-US"))
                .with_locale((
                    langid!("en-US"),
                    include_str!("../resources/i18n/en-US.ftl"),
                ))
                .with_locale((
                    langid!("uk-UA"),
                    include_str!("../resources/i18n/uk-UA.ftl"),
                ))
        });

        spawn(async move {
            if let Ok(Value::Bool(false)) = eval(include_str!("../resources/js/sw_register.js"))
                .recv()
                .await
            {
                alert_dialog!("error-service-worker-caption", "error-service-worker-text")
            }
        });

        rsx! {
            document::Stylesheet {
                href: asset!("resources/css/main.css")
            }
            Router::<Route> {}
        }
    })
}

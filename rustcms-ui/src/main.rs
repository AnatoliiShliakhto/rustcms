#![forbid(unsafe_code)]
#![allow(non_snake_case)]
use ::dioxus::{
    logger::{init as logger_init, tracing::Level},
    prelude::*,
};

fn main() {
    logger_init(Level::INFO).expect("failed to init logger");

    launch(|| {
        rsx! {
            link { rel: "stylesheet", href: "./assets/tailwind.css" }
            main {
                "Hello there!"
            }
        }
    })
}

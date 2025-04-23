use ::dioxus::prelude::*;
use ::dioxus_i18n::t;

pub fn Loading() -> Element {
    rsx! {
        div {
            class: "div-full",
            div {
                class: "inline-flex items-center gap-3",
                span {
                    class: "loading loading-bars loading-lg"
                }
                span {
                    { t!("action-loading") }
                }
            }
        }
    }
}
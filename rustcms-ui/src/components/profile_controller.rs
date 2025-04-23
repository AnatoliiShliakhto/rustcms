use ::dioxus::prelude::*;

use crate::components::*;

pub fn ProfileController() -> Element {
    rsx! {
        Link {
            class: "btn btn-ghost join-item",
            //onclick: move |_| state_fn!(search_engine_clear),
            to: "/account/sign-in",
            Icon { icon: Icons::SignIn, class: "size-6" }
        }
    }
}
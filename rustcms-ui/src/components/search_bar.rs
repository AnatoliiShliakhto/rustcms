use ::dioxus::prelude::*;
use ::dioxus_i18n::tid;

use crate::components::{Icon, Icons};

pub fn SearchBar() -> Element {
    rsx! {
        button {
            class: "flex inline-flex items-center",
            class: "rounded-full px-2 py-1 inset-ring cursor-pointer",
            Icon { icon: Icons::Search, class: "size-5" }
            span {
                class: "mx-4 flex-1",
                { tid!("search-label") }
            }
            kbd {
                class: "hidden kbd kbd-md [.os-macos_&]:block",
                "⌘K"
            }
            kbd {
                class: "hidden kbd kbd-md not-[.os-macos_&]:block",
                "Ctrl K"
            }
        }
    }
}

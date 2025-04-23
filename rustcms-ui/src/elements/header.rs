use ::dioxus::prelude::*;
use ::dioxus_i18n::tid;

use crate::components::*;

pub fn Header() -> Element {
    rsx! {
        nav {
            class: "navbar min-h-0 px:auto gap-2",
            div {
                class: "flex flex-1 flex-nowrap items-center md:gap-1 lg:gap-2",
                label {
                    class: "btn btn-square btn-ghost drawer-button lg:hidden",
                    r#for: "main-menu",
                    Icon {
                        icon: Icons::Menu,
                        class: "size-6",
                    }
                }
                div {
                    class: "items-center gap-2 hidden lg:flex",
                    class: "flex-nowrap text-xl font-semibold",
                    { tid!("site-title") }
                }                
            }
            div {
                class: "flex",
                SearchBar {}   
            }            
            div {
                class: "flex join",
                ThemeController {}
                LanguageController {}
                ProfileController {}
            }
        }
    }
}
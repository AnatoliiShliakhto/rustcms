use ::dioxus::prelude::*;
use ::dioxus_i18n::t;

use crate::app::Route;
use crate::{elements::Footer, components::*};

pub fn MainMenu() -> Element {
    rsx! {
        aside {
            class: "flex flex-col bg-base-100 min-h-screen w-85",
            div {
                class: "bg-base-100/90 navbar sticky top-0 z-20 hidden items-center justify-center",
                class: "gap-4 px-4 py-2 backdrop-blur lg:flex",
                div {
                    class: "size-10 bg-info",
                    style: "-webkit-mask: url('/assets/logo.svg') no-repeat center;",
                    style: "url('/assets/logo.svg') no-repeat center;",
                }
                span {
                    class: "flex flex-nowrap text-3xl",
                    class: "items-center font-semibold text-info",
                    { t!("site-short-title") }
                }
            }
            div { class: "h-4" }
            ul {
                class: "menu w-full px-4 py-0",
                li {
                    Link {
                        to: Route::Home {},
                        Icon { icon: Icons::Home, class: "size-6" }
                        { t!("menu-home") }
                    }
                }
                li {}
                li {
                    Link {
                        to: Route::SignIn {},
                        Icon { icon: Icons::SignIn, class: "size-6" }
                        { t!("menu-sign-in") }
                    }
                }
            }
            div {
                class: "bg-base-100 pointer-events-none sticky bottom-0 flex h-40",
                class: "[mask-image:linear-gradient(transparent,#000000)]",
            }
            div { class: "flex flex-col grow justify-end",
                Footer {}
            }
        }
    }
}
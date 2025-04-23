use ::dioxus::prelude::*;
use ::dioxus_i18n::tid;

pub fn Footer() -> Element {
    
    rsx! {
        footer { 
            class: "footer footer-center h-14 p-4 self-center",
            a {
                class: "link link-hover text-neutral hover:text-accent text-sm",
                href: "https://github.com/AnatoliiShliakhto/rustcms",
                target: "_blank",
                { tid!("site-copyright") }
            }
        }
    }
}
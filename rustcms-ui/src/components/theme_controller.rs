use ::dioxus::prelude::*;
use ::dioxus_i18n::tid;

use crate::{components::*, services::use_persistent, THEMES};

pub fn ThemeController() -> Element {
    if THEMES.len() < 2 { return rsx!() }
    
    let mut theme_persistent = use_persistent("theme", || THEMES[0].to_string());

    let change_theme = move |event: Event<FormData>| {
        if let Some(FormValue(value)) = event.values().get("theme-dropdown") {
            (!value.is_empty()).then(|| theme_persistent.set(value[0].clone()));
        }
    };

    rsx! {
        div {
            class: "dropdown dropdown-end block",
            title: tid!("theme-change"),
            button {
                class: "btn btn-ghost join-item",
                tabindex: 0,
                Icon { icon: Icons::Theme, class: "size-6 stroke-current" }
            }
            ul {
                class: "dropdown-content bg-base-200 text-base-content rounded-box",
                class: "top-px max-h-[calc(100vh-11rem)] w-52 overflow-y-hidden",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 mt-16 z-1",
                tabindex: 0,
                form {
                    onchange: change_theme,

                    for theme in THEMES.iter() {
                        li {
                            div {
                                class: "bg-base-100 grid shrink-0 grid-cols-2 gap-0.5 rounded-md p-1 shadow-sm",
                                class: "fixed mt-2.75 right-3 z-10",
                                "data-theme": *theme,
                                div { class: "bg-base-content size-1 rounded-full" }
                                div { class: "bg-primary size-1 rounded-full" }
                                div { class: "bg-secondary size-1 rounded-full" }
                                div { class: "bg-accent size-1 rounded-full" }
                            }
                            input {
                                class: "theme-controller btn btn-block btn-ghost justify-start",
                                r#type: "radio",
                                name: "theme-dropdown",
                                value: *theme,
                                initial_checked: theme_persistent.get().eq(*theme),
                                aria_label: tid!(&format!("theme-{theme}")),
                            }
                        }
                    }
                }
            }
        }
    }
}

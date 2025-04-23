use ::dioxus::prelude::*;
use ::dioxus_i18n::{prelude::*, tid};

use crate::{LANGUAGES, components::*, services::use_persistent};

pub fn LanguageController() -> Element {
    if LANGUAGES.len() < 2 { return rsx!() }
    
    let mut i18n = i18n();
    let mut lang_persistent = use_persistent("lang", || LANGUAGES[0].to_string());

    let change_language = move |event: Event<FormData>| {
        if let Some(FormValue(value)) = event.values().get("language-dropdown") {
            (!value.is_empty()).then(|| {
                i18n.set_language(value[0].parse().unwrap_or_default());
                lang_persistent.set(value[0].clone());
            });
        }
    };

    use_hook(|| i18n.set_language(lang_persistent.get().parse().unwrap_or_default()));

    rsx! {
        div {
            class: "dropdown dropdown-end block",
            title: tid!("language-change"),
            button {
                class: "btn btn-ghost join-item",
                tabindex: 0,
                Icon { icon: Icons::Language, class: "size-6" }
            }
            ul {
                class: "dropdown-content bg-base-200 text-base-content rounded-box",
                class: "top-px max-h-[calc(100vh-11rem)] w-52 overflow-y-hidden",
                class: "border border-white/5 shadow-2xl outline-1 outline-black/5 mt-16 z-1",
                tabindex: 0,
                form {
                    onchange: change_language,

                    for language in LANGUAGES.iter() {
                        li {
                            input {
                                class: "w-full btn btn-block btn-ghost justify-start",
                                r#type: "radio",
                                name: "language-dropdown",
                                value: *language,
                                aria_label: tid!(&format!("language-{language}")),
                            }
                        }
                    }
                }
            }
        }
    }
}

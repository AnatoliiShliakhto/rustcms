use ::dioxus::prelude::*;

use crate::{
    app::Route,
    components::Dialog,
    elements::{Header, MainMenu},
    pages::Loading,
};

pub fn Layout() -> Element {
    rsx! {
        div {
            class: "bg-base-100 drawer mx-auto max-w-[100rem] min-h-screen lg:drawer-open",
            input {
                class: "drawer-toggle",
                id: "main-menu",
                r#type: "checkbox",
            }
            div {
                class: "drawer-content",
                div {
                    class: "bg-base-100/90 text-base-content sticky top-0 z-30",
                    class: "flex h-12 w-full [transform:translate3d(0,0,0)] justify-center",
                    class: "backdrop-blur transition-shadow duration-100 print:hidden",
                    Header {}
                }
                div {
                    class: "relative max-w-[100vw] px-6 pb-16 xl:pe-2",
                    SuspenseBoundary {
                        fallback: |_context: SuspenseContext| rsx! {
                            Loading {}
                        },
                        Outlet::<Route> {}
                    }
                }
            }
            div {
                class: "drawer-side z-40",
                style: "scroll-behavior: smooth; scroll-padding-top: 5rem;",
                label {
                    class: "drawer-overlay",
                    r#for: "main-menu",
                    aria_label: "close sidebar",
                    MainMenu {}
                }
            }
        }
        Dialog {}
    }
}

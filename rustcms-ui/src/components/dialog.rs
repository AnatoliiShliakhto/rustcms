use ::dioxus::prelude::*;
use ::dioxus_i18n::tid;

use crate::{app::use_state, components::icons::*, hide_dialog};

#[allow(dead_code)]
#[derive(Clone, PartialEq)]
pub enum MessageKind {
    Alert,
    Success,
}

#[derive(Clone, PartialEq)]
pub struct DialogArgs {
    pub kind: MessageKind,
    pub caption: String,
    pub message: String,
    pub handler: Option<EventHandler<MouseEvent>>,
}

pub fn Dialog() -> Element {
    let Some(args) = use_state().dialog_args.read().clone() else {
        return rsx!()
    };

    rsx! {
        dialog {
            class: "modal modal-open",
            onclick: move |_| hide_dialog!(),
            div {
                class: "modal-box p-0",
                onclick: move |event| if args.handler.is_some() {
                    event.stop_propagation()
                },
                div {
                    class: "flex flex-col m-6 items-center",
                    div {
                        class: "mx-auto flex size-12 shrink-0 items-center justify-center",
                        class: "rounded-full bg-base-200 sm:mx-0 sm:size-10",
                        Icon {
                            icon: Icons::Alert,
                            class: "size-6 text-error"
                        }
                    }
                    h3 {
                        class: "text-lg font-semibold pt-3",
                        { args.caption }
                    }
                    p {
                        class: "flex-1 py-3",
                        { args.message }
                    }
                    div {
                        class: "flex w-full bg-base-200 justify-end gap-4 p-3",
                        button {
                            class: "btn btn-neutral",
                            "Cancel"
                        }
                        button {
                            class: "btn btn-error",
                            "error"
                        }
                    }
                }
            }
        }
    }
}    
    /*    
    rsx! {
        section {
            class: "modal modal-open",
            class: if args.on_yes.is_none() && args.on_no.is_none() { "cursor-pointer" },
            onclick: move |_| hide_dialog!(),
            div {
                class: "modal-box",
                onclick: move |event| if args.on_yes.is_some() || args.on_no.is_some() {
                    event.stop_propagation()
                },

                match args.kind {
                    MessageKind::Alert => rsx! {
                        div {
                            class: "flex gap-4 text-lg font-bold wrap",
                            Icon { icon: Icons::Alert, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { args.caption }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Info => rsx! {
                        div {
                            class: "flex gap-4 text-lg font-bold wrap text-info",
                            Icon { icon: Icons::Info, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { args.caption }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Error => rsx! {
                        div {
                            class: "flex gap-4 text-lg font-bold wrap text-error",
                            Icon { icon: Icons::Error, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { args.caption }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Success => rsx! {
                        div {
                            class: "flex gap-4 text-lg font-bold wrap text-success",
                            Icon { icon: Icons::Success, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { args.caption }
                                div { class: "divider my-0" }
                            }
                        }
                    },
                    MessageKind::Warning => rsx! {
                        div {
                            class: "flex gap-4 text-lg font-bold wrap text-warning",
                            Icon { icon: Icons::Warning, class: "stroke-current shrink-0 size-10" }
                            div {
                                class: "flex grow flex-col",
                                { args.caption }
                                div { class: "divider my-0" }
                            }
                        }
                    }
                }

                p {
                    class: "indent-14 whitespace-pre-line",
                    { args.message }
                }
                div {
                    class: "card-actions mt-6 gap-6 justify-end",
                    if let Some(handler) = args.on_yes {
                        button {
                            class: match args.kind {
                                MessageKind::Alert => "btn btn-primary",
                                MessageKind::Info => "btn btn-info",
                                MessageKind::Error => "btn btn-error",
                                MessageKind::Success => "btn btn-success",
                                MessageKind::Warning => "btn btn-warning",
                            },
                            onclick: move |event| {
                                hide_dialog!();
                                handler(event)
                            },
                            { tid!("action-yes") }
                        }
                    }
                    if let Some(handler) = args.on_no {
                        button {
                            class: "btn btn-outline",
                            onclick: move |event| {
                                hide_dialog!();
                                handler(event)
                            },
                            { tid!("action-no") }
                        }
                    }
                }
            }
        }
    }
    
}
 */

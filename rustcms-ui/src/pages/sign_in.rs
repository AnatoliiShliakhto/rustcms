use ::dioxus::prelude::*;
use ::dioxus_i18n::tid;

use crate::{app::Route, get, services::FormEventExt};

pub fn SignIn() -> Element {
    let sign_in_action = move |event: FormEvent| {
        if let (Some(login), Some(password)) = (event.value("login"), event.value("password")) {
            let url = format!("/api/v1/auth?login={login}&password={password}");

            spawn(async move {
                get!(&url).then(|| navigator().replace(Route::Home {}));
            });
        };
    };

    rsx! {
        div {
            class: "div-full",
            div {
                class: "hero",
                div {
                    class: "hero-content flex-col lg:flex-row-reverse w-full",
                    div {
                        class: "text-center lg:text-left w-full max-w-sm",
                        h1 {
                            class: "text-3xl font-bold",
                            { tid!("page-sign-in-header") }
                        }
                        p {
                            class: "py-6",
                            { tid!("page-sign-in-announcement") }
                        }
                    }

                    div {
                        class: "card bg-base-100 card-border w-full max-w-sm shrink-0 shadow-xl",
                        div {
                            class: "card-body pt-4",
                            form {
                                id: "sign-in-form",
                                autocomplete: "off",
                                "onsubmit": "event.preventDefault()",
                                onsubmit: sign_in_action,
                                label {
                                    class: "floating-label mt-4",
                                    span { { tid!("label-login") } }
                                    input {
                                        class: "input validator w-full",
                                        r#type: "text",
                                        name: "login",
                                        placeholder: tid!("placeholder-login"),
                                        required: true,
                                        minlength: 4,
                                        maxlength: 50
                                    }
                                }
                                label {
                                    class: "floating-label mt-5",
                                    span { { tid!("label-password") } }
                                    input {
                                        class: "input validator w-full",
                                        r#type: "password",
                                        name: "password",
                                        placeholder: tid!("placeholder-password"),
                                        required: true,
                                        minlength: 4,
                                        maxlength: 30
                                    }
                                }
                            }
                            div {
                                class: "flex justify-end",
                                a {
                                    class: "link link-hover",
                                    { tid!("msg-forgot-password") }
                                }
                            }
                            button {
                                class: "btn btn-neutral mt-4",
                                form: "sign-in-form",
                                { tid!("action-sign-in") }
                            }
                            div {
                                class: "divider m-1",
                                { tid!("label-divider") }
                            }
                            button {
                                class: "btn",
                                { tid!("action-create-new-account") }
                            }
                        }
                    }
                }
            }
        }
    }
}

use ::dioxus::prelude::*;

use crate::{elements::Layout, pages::*};

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
#[allow(clippy::enum_variant_names)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Route::Home {})]
    Home {},
    #[route("/account/sign-in")]
    SignIn {},
}

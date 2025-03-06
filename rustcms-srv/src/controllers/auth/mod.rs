mod authorize;
mod token;
mod revoke;

static TAG_AUTHORIZATION: &str = "Authorization";

pub use self::{
    authorize::*,
    token::*,
    revoke::*,
};
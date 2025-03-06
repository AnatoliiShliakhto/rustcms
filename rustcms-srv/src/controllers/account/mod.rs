mod register;
mod delete;

static TAG_ACCOUNT: &str = "Account";

pub use self::{
    register::*,
    delete::*,
};
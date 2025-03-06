mod errors;
mod state;
mod api;
pub(crate) mod router;

pub(crate) use ::srv_macros::handler;

pub(crate) use self::{errors::*, state::AppState};
pub(crate) type Result<T> = std::result::Result<T, Error>;

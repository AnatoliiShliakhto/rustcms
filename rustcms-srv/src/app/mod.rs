mod errors;
mod state;
mod api;
pub(crate) mod router;

use ::surrealdb::{Surreal, engine::any::Any};

pub(crate) use ::srv_macros::handler;

pub(crate) use self::{errors::*, state::AppState};

pub(crate) type Database = Surreal<Any>;


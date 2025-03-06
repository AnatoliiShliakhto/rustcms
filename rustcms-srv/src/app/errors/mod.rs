mod auth;
mod database;
mod error;

pub use self::{
    auth::AuthError,
    database::DatabaseError,
    error::{Error, ErrorBody},
};
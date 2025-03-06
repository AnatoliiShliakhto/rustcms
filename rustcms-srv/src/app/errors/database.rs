use ::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error::IntoErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Entry not found")]
    EntryNotFound,
    #[error("Entry not created")]
    EntryNotCreated,
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_error_response()
    }
}

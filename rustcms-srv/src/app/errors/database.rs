use ::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error::IntoErrorResponse;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Record not found")]
    NotFound,
    #[error("Record creation failed")]
    Create,
    #[error("Record update failed")]
    Update,
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_error_response()
    }
}

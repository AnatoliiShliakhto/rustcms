use ::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error::IntoErrorResponse;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Record not found")]
    RecordNotFound,
    #[error("Record not crated")]
    RecordNotCreated,
}

impl IntoResponse for DatabaseError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_error_response()
    }
}

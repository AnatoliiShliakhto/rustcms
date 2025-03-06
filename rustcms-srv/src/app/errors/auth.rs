use ::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error::IntoErrorResponse;

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::Unauthorized |
            Self::TokenExpired |
            Self::WrongCredentials => StatusCode::UNAUTHORIZED,
            Self::InvalidToken |
            Self::MissingCredentials => StatusCode::BAD_REQUEST,
            Self::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status_code, self).into_error_response()
    }
}

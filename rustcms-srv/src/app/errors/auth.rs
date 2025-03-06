use ::axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::error::IntoErrorResponse;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Access forbidden")]
    AccessForbidden,
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing token")]
    MissingToken,
    #[error("Token creation error")]
    TokenCreation,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::Unauthorized |
            Self::WrongCredentials |
            Self::InvalidToken |
            Self::MissingToken => StatusCode::UNAUTHORIZED,
            Self::TokenCreation => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AccessForbidden => StatusCode::FORBIDDEN,
        };

        (status_code, self).into_error_response()
    }
}

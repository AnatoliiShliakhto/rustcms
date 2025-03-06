use ::axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use ::serde::Serialize;
use ::serde_json::Error as SerdeError;
use ::std::{borrow::Cow, io::Error as IoError};
use ::surrealdb::Error as SurrealDbError;
use ::tracing::{error, warn};
use ::validator::ValidationErrors;

use super::{AuthError, DatabaseError};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),
    #[error(transparent)]
    AuthError(#[from] AuthError),
    #[error(transparent)]
    ValidationErrors(#[from] ValidationErrors),
    #[error[transparent]]
    JsonRejection(#[from] JsonRejection),
    #[error[transparent]]
    IoError(#[from] IoError),
    #[error[transparent]]
    SurrealDbError(#[from] SurrealDbError),
    #[error[transparent]]
    SerdeError(#[from] SerdeError),
    #[error("{0}")]
    CustomError(&'static str),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DatabaseError(err) => err.into_response(),
            Error::AuthError(err) => err.into_response(),
            Error::ValidationErrors(_) => {
                warn!("{}", format!("{self}").replace('\n', " "));
                (StatusCode::BAD_REQUEST, "The payload isn't valid").into_error_response()
            }
            Error::JsonRejection(_) => (StatusCode::BAD_REQUEST, self).into_error_response(),
            Error::CustomError(err) => (StatusCode::BAD_REQUEST, err).into_error_response(),
            _ => {
                error!("{self}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_error_response()
            }
        }
    }
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorBody<'a> {
    pub error: Cow<'a, str>,
}

impl<T: ToString> From<T> for ErrorBody<'_> {
    fn from(value: T) -> Self {
        Self {
            error: Cow::Owned(value.to_string()),
        }
    }
}

pub trait IntoErrorResponse {
    fn into_error_response(self) -> Response;
}

impl<T: ToString> IntoErrorResponse for (StatusCode, T) {
    fn into_error_response(self) -> Response {
        (
            self.0,
            Json(ErrorBody {
                error: Cow::Owned(self.1.to_string()),
            }),
        )
            .into_response()
    }
}

use ::axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ::serde::Serialize;
use ::serde_json::{Error as SerdeError, Value};
use ::std::{borrow::Cow, collections::HashMap, io::Error as IoError};
use ::surrealdb::Error as SurrealDbError;
use ::tracing::error;
use ::validator::{ValidationErrors, ValidationErrorsKind};

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

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorBody<'a> {
    pub error: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Value>,
}

pub trait IntoErrorResponse {
    fn into_error_response(self) -> Response;
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DatabaseError(err) => err.into_response(),
            Error::AuthError(err) => err.into_response(),
            Error::ValidationErrors(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "The payload isn't valid",
                normalize_validator_errors(err),
            )
                .into_error_response(),
            Error::JsonRejection(_) => (StatusCode::BAD_REQUEST, self).into_error_response(),
            Error::CustomError(err) => (StatusCode::BAD_REQUEST, err).into_error_response(),
            _ => {
                error!("{self}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_error_response()
            }
        }
    }
}

impl<T: ToString> From<T> for ErrorBody<'_> {
    fn from(value: T) -> Self {
        Self {
            error: Cow::Owned(value.to_string()),
            details: None,
        }
    }
}

impl<T: ToString> IntoErrorResponse for (StatusCode, T) {
    fn into_error_response(self) -> Response {
        (
            self.0,
            Json(ErrorBody {
                error: Cow::Owned(self.1.to_string()),
                details: None,
            }),
        )
            .into_response()
    }
}

impl<T: ToString, U: Serialize> IntoErrorResponse for (StatusCode, T, U) {
    fn into_error_response(self) -> Response {
        (
            self.0,
            Json(ErrorBody {
                error: Cow::Owned(self.1.to_string()),
                details: Some(serde_json::to_value(self.2).unwrap_or_default()),
            }),
        )
            .into_response()
    }
}

fn normalize_validator_errors(errors: ValidationErrors) -> HashMap<String, String> {
    let mut normalized = HashMap::new();

    for (field, error) in errors.errors().iter() {
        if let ValidationErrorsKind::Field(err) = error {
            let mut messages = Vec::new();
            err.iter().for_each(|x| {
                if let Some(message) = &x.message {
                    messages.push(message.to_string());
                }
                normalized.insert(field.to_string(), messages.join(", "));
            });
        }
    }
    
    normalized
}

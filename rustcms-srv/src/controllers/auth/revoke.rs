use ::axum::{extract::State, http::StatusCode, response::IntoResponse};
use ::serde::Deserialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*,
    repositories::middleware::TokenRepository,
    services::middleware::{Claims, ValidatedJson},
};

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct RevokeRefreshTokenPayload<'a> {
    #[validate(length(min = 7))]
    pub refresh_token: Cow<'a, str>,
}

use super::TAG_AUTHORIZATION;

#[utoipa::path(
    delete,
    path = "/v1/auth/token",
    tag = TAG_AUTHORIZATION,
    request_body(
        description = "Revoke the Refresh Token",
        content = RevokeRefreshTokenPayload,
    ),
    responses(
        (
            status = OK, 
            description = "The Refresh Token is revoked",
        ),
        (
            status = UNAUTHORIZED,
            description = "The Refresh Token is expired",
            body = ErrorBody,
        ),
        (
            status = BAD_REQUEST,
            description = "The Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
pub async fn revoke(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RevokeRefreshTokenPayload<'_>>,
) -> Result<impl IntoResponse> {
    let refresh_token_id =
        Claims::from_refresh_token(&payload.refresh_token, &state.keys.decoding)?
            .jti
            .unwrap();
    state
        .db
        .delete_refresh_token(refresh_token_id)
        .await?;

    Ok(StatusCode::OK)
}

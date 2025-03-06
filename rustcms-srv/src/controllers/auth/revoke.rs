use ::axum::extract::State;
use ::serde::Deserialize;
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*,
    repositories::middleware::TokenRepository,
    services::middleware::{Claims, ValidatedJson},
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct RevokeRefreshTokenPayload<'a> {
    #[validate(length(min = 7, max = 100))]
    pub refresh_token: Cow<'a, str>,
}

#[utoipa::path(
    delete,
    path = "/v1/auth/revoke",
    tag = super::TAG_AUTHORIZATION,
    request_body(
        description = "Revoke refresh token",
        content = RevokeRefreshTokenPayload,
    ),
    responses(
        (
            status = OK, 
            description = "Refresh token was revoked",
        ),
        (
            status = UNAUTHORIZED,
            description = "Refresh token is expired",
            body = ErrorBody,
        ),
        (
            status = BAD_REQUEST,
            description = "Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
#[handler(result)]
pub async fn revoke(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RevokeRefreshTokenPayload<'_>>,
) {
    let refresh_token_id =
        Claims::from_refresh_token(&payload.refresh_token, &state.cfg.jwt_keys.decoding)?
            .jti
            .unwrap_or_default();
    state
        .db
        .delete_refresh_token(refresh_token_id)
        .await?;
}

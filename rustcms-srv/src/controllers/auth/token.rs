use ::axum::{extract::State, Json};
use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*,
    repositories::{account::AccountAuthRepository, middleware::TokenRepository},
    services::middleware::{Claims, ValidatedJson},
};

#[derive(Deserialize, ToSchema, Validate)]
pub struct RefreshTokenPayload<'a> {
    #[validate(length(min = 7))]
    pub refresh_token: Cow<'a, str>,
}

#[derive(Serialize, ToSchema)]
pub struct RefreshTokenBody<'a> {
    pub token_type: &'a str,
    pub access_token: Cow<'a, str>,
    pub refresh_token: Cow<'a, str>,
}

#[utoipa::path(
    post,
    path = "/v1/auth/token",
    tag = super::TAG_AUTHORIZATION,
    request_body(
        description = "Renew Access Token by Refresh Token",
        content = RefreshTokenPayload,
    ),
    responses(
        (
            status = OK, 
            description = "Returns the authorized tokens",
            body = RefreshTokenBody,
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
#[handler]
pub async fn token(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<RefreshTokenPayload<'_>>,
) {
    let current_refresh_token_id =
        Claims::from_refresh_token(&payload.refresh_token, &state.cfg.jwt_keys.decoding)?
            .jti
            .unwrap();

    let auth = state
        .db
        .find_auth_by_token(current_refresh_token_id.clone())
        .await?;

    let refresh_token_id = state
        .db
        .update_refresh_token(current_refresh_token_id, state.cfg.jwt_refresh_expiration)
        .await?;
    let refresh_token = Claims::new()
        .id(&refresh_token_id)
        .issuer(&state.cfg.jwt_issuer)
        .subject(&state.cfg.jwt_subject)
        .expiration_days(state.cfg.jwt_refresh_expiration)
        .build_token(&state.cfg.jwt_keys.encoding)?;

    // Build the access token
    let access_token = Claims::new()
        .issuer(&state.cfg.jwt_issuer)
        .subject(&state.cfg.jwt_subject)
        .expiration_minutes(state.cfg.jwt_access_expiration)
        .auth(auth)
        .build_token(&state.cfg.jwt_keys.encoding)?;

    Ok(Json(RefreshTokenBody {
        token_type: "Bearer",
        access_token,
        refresh_token,
    }))
}

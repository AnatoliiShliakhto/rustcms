use ::axum::{extract::State, response::IntoResponse, Json};
use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::ToSchema;
use ::validator::Validate;

use crate::{
    app::*,
    repositories::{accounts::AccountAuthRepository, middleware::TokenRepository},
    services::middleware::{Claims, ValidatedJson},
};

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct AuthPayload<'a> {
    #[validate(length(min = 4, max = 50))]
    pub login: Cow<'a, str>,
    #[validate(length(min = 4, max = 30))]
    pub password: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthBody<'a> {
    pub token_type: &'a str,
    pub access_token: Cow<'a, str>,
    pub refresh_token: Cow<'a, str>,
}

use super::TAG_AUTHORIZATION;

#[utoipa::path(
    post,
    path = "/v1/auth",
    tag = TAG_AUTHORIZATION,
    request_body(
        description = "API authorization endpoint",
        content = AuthPayload,
    ),
    responses(
        (
            status = OK, 
            description = "Returns the authorized tokens",
            body = AuthBody,
        ),
        (
            status = UNAUTHORIZED,
            description = "Wrong credentials",
            body = ErrorBody,
        ),
        (
            status = BAD_REQUEST,
            description = "The Payload isn't valid",
            body = ErrorBody,
        ),
    ),
)]
pub async fn authorize(
    State(state): State<Arc<AppState>>,
    ValidatedJson(payload): ValidatedJson<AuthPayload<'_>>,
) -> Result<impl IntoResponse> {
    // Get the auth state by credentials
    let auth = state
        .db
        .find_auth_by_credentials(payload.login, payload.password)
        .await?;

    // Build the refresh token
    let refresh_token_id = state
        .db
        .create_refresh_token(&auth.id, state.cfg.jwt_refresh_expiration, payload.device)
        .await?;
    let refresh_token = Claims::new()
        .id(&refresh_token_id)
        .issuer(&state.cfg.jwt_issuer)
        .subject(&state.cfg.jwt_subject)
        .expiration_days(state.cfg.jwt_refresh_expiration)
        .build_token(&state.keys.encoding)?;

    // Build the access token
    let access_token = Claims::new()
        .issuer(&state.cfg.jwt_issuer)
        .subject(&state.cfg.jwt_subject)
        .expiration_minutes(state.cfg.jwt_access_expiration)
        .auth(auth)
        .build_token(&state.keys.encoding)?;

    // Send the authorized tokens
    Ok(Json(AuthBody {
        token_type: "Bearer",
        access_token,
        refresh_token,
    }))
}

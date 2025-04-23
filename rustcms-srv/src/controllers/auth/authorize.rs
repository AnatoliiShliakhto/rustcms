use ::axum::{extract::{Query, State}, http::header::SET_COOKIE, response::AppendHeaders, Json};
use ::axum_extra::extract::CookieJar;
use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, sync::Arc};
use ::utoipa::{IntoParams, ToSchema};

use crate::{
    app::*,
    repositories::{accounts::AccountAuthRepository, middleware::TokenRepository},
    services::middleware::*,
};

#[derive(Deserialize, IntoParams)]
pub struct AuthPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Cow<'a, str>>,
}

#[derive(Serialize, ToSchema)]
pub struct AuthBody<'a> {
    pub token_type: &'a str,
    pub access_token: Cow<'a, str>,
}

#[utoipa::path(
    get,
    path = "/v1/auth",
    tag = super::TAG_AUTHORIZATION,
    params(
        AuthPayload
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
    ),
)]
#[handler]
pub async fn authorize(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
    Query(payload): Query<AuthPayload<'_>>,
) {
    let refresh_token;

    let auth = if payload.login.is_none() {
        let Some(current_refresh_token) = jar.get("RT_UUID") else {
            Err(AuthError::MissingToken)?
        };
        if current_refresh_token.value().is_empty() {
            Err(AuthError::InvalidToken)?
        }

        let auth = state.database.find_auth_by_token(current_refresh_token.value()).await?;

        refresh_token = state
            .database
            .update_refresh_token(current_refresh_token.value(), state.config.security.jwt.refresh_expiration)
            .await?;

        auth
    } else {
        let AuthPayload { login: Some(login), password: Some(password), device } = payload else {
            Err(AuthError::WrongCredentials)?
        };

        let auth = state
            .database
            .find_auth_by_credentials(login, password)
            .await?;

        refresh_token = state
            .database
            .create_refresh_token(&auth.id, state.config.security.jwt.refresh_expiration, device)
            .await?;

        auth
    };

    // Build the access token
    let access_token = Claims::new()
        .issuer(&state.config.security.jwt.issuer)
        .subject(&state.config.security.jwt.subject)
        .expiration_minutes(state.config.security.jwt.access_expiration)
        .auth(auth)
        .build_token(&state.config.security.jwt.keys.encoding)?;

    // Send the authorized tokens
    Ok((
        AppendHeaders(vec![(
            SET_COOKIE,
            format!(
                "RT_UUID={refresh_token}; Path=/api/v1/auth; Max-Age={0}; {1}",
                state.config.security.jwt.refresh_expiration * 24 * 60 * 60,
                state.config.security.set_cookie,
            )
        )]),
        Json(AuthBody {
            token_type: "Bearer",
            access_token,
        })))
}

use ::axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use ::axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use ::chrono::{Duration, Utc};
use ::jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use ::serde::{Deserialize, Serialize};
use ::std::{borrow::Cow, sync::Arc};

use super::AuthState;
use crate::app::*;

const JWT_EXPIRATION: usize = 600usize; // 10 minutes

#[derive(Serialize, Deserialize)]
pub struct Claims<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iss: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<Cow<'a, str>>,
    pub iat: usize,
    pub exp: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<AuthState<'a>>,
}

#[allow(dead_code)]
impl<'a> Claims<'a> {
    pub fn new() -> Self {
        let timestamp_now = Utc::now().timestamp() as usize;

        Self {
            iss: None,
            sub: None,
            jti: None,
            iat: timestamp_now,
            exp: timestamp_now + JWT_EXPIRATION,
            auth: None,
        }
    }

    pub fn issuer(mut self, iss: &'a str) -> Self {
        self.iss = Some(Cow::Borrowed(iss));
        self
    }

    pub fn subject(mut self, sub: &'a str) -> Self {
        self.sub = Some(Cow::Borrowed(sub));
        self
    }

    pub fn id(mut self, id: &'a str) -> Self {
        self.jti = Some(Cow::Borrowed(id));
        self
    }

    pub fn expiration(mut self, timestamp: usize) -> Self {
        self.exp = timestamp;
        self
    }

    pub fn expiration_minutes(mut self, minutes: i64) -> Self {
        self.exp = (Utc::now() + Duration::minutes(minutes)).timestamp() as usize;
        self
    }

    pub fn expiration_hours(mut self, hours: i64) -> Self {
        self.exp = (Utc::now() + Duration::hours(hours)).timestamp() as usize;
        self
    }

    pub fn expiration_days(mut self, days: i64) -> Self {
        self.exp = (Utc::now() + Duration::days(days)).timestamp() as usize;
        self
    }

    pub fn auth(mut self, auth: AuthState<'a>) -> Self {
        self.auth = Some(auth);
        self
    }

    pub fn is_expired(&self) -> bool {
        self.exp < Utc::now().timestamp() as usize
    }

    pub fn build_token(&self, encoding_key: &EncodingKey) -> Result<Cow<'static, str>, AuthError> {
        encode(&Header::default(), self, encoding_key)
            .map_err(|_| AuthError::TokenCreation)
            .map(|v| Ok(Cow::Owned(v)))?
    }

    pub fn from_refresh_token(token: &str, decoding_key: &DecodingKey) -> Result<Self, AuthError> {
        let token_data = decode::<Claims>(token, decoding_key, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.jti.is_none() {
            Err(AuthError::InvalidToken)?
        }

        Ok(token_data.claims)
    }

    pub fn account_id(&self) -> Option<&str> {
        self.auth.as_ref().map(|v| v.id.as_ref())
    }
}

impl Default for Claims<'_> {
    fn default() -> Self {
        let timestamp_now = Utc::now().timestamp() as usize;

        Self {
            iss: None,
            sub: None,
            jti: None,
            iat: timestamp_now,
            exp: timestamp_now + JWT_EXPIRATION,
            auth: Some(AuthState::default()),
        }
    }
}

impl<S> FromRequestParts<S> for Claims<'_>
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync + Clone,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state: Arc<AppState> = FromRef::from_ref(state);

        let Ok(TypedHeader(Authorization(bearer))) =
            parts.extract::<TypedHeader<Authorization<Bearer>>>().await
        else {
            Err(AuthError::MissingToken)?
        };

        let token_data = decode::<Claims>(
            bearer.token(),
            &state.config.security.jwt.keys.decoding,
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.auth.is_none() {
            Err(AuthError::InvalidToken)?
        }

        Ok(token_data.claims)
    }
}

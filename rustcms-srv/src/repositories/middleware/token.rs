use ::chrono::{Duration, Utc};
use ::std::borrow::Cow;
use ::surrealdb::{Surreal, engine::any::Any};

use crate::app::*;

pub trait TokenRepository {
    async fn create_refresh_token(
        &self,
        account_id: impl AsRef<str>,
        expiration_days: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>>;
    async fn update_refresh_token(
        &self,
        token_id: impl AsRef<str>,
        expiration_days: i64,
    ) -> Result<Cow<str>>;
    async fn delete_refresh_token(&self, token_id: impl AsRef<str>) -> Result<()>;
}

impl TokenRepository for Surreal<Any> {
    async fn create_refresh_token(
        &self,
        account_id: impl AsRef<str>,
        expiration_days: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>> {
        self.query(include_str!(
            "../../../resources/queries/middleware/create_refresh_token.surql"
        ))
        .bind(("account_id", account_id.as_ref().to_string()))
        .bind((
            "expiration_at",
            (Utc::now() + Duration::days(expiration_days)).timestamp(),
        ))
        .bind(("device", device.map(|v| v.to_string())))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(AuthError::TokenCreation.into())
    }

    async fn update_refresh_token(
        &self,
        token_id: impl AsRef<str>,
        expiration_days: i64,
    ) -> Result<Cow<str>> {
        self.query(include_str!(
            "../../../resources/queries/middleware/update_refresh_token.surql"
        ))
        .bind(("token_id", token_id.as_ref().to_string()))
        .bind((
            "expiration_at",
            (Utc::now() + Duration::days(expiration_days)).timestamp(),
        ))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(AuthError::TokenCreation.into())
    }

    async fn delete_refresh_token(&self, token_id: impl AsRef<str>) -> Result<()> {
        self.query(include_str!(
            "../../../resources/queries/middleware/delete_refresh_token.surql"
        ))
        .bind(("token_id", token_id.as_ref().to_string()))
        .await?
        .check()?;

        Ok(())
    }
}

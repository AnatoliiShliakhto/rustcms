use ::chrono::{Duration, Utc};
use ::std::borrow::Cow;
use ::surrealdb::{engine::any::Any, Surreal};

use crate::app::*;

pub trait TokenRepository<S> {
    async fn create_refresh_token(
        &self,
        account_id: S,
        expiration_days: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>>;
    async fn update_refresh_token(&self, token_id: S, expiration_days: i64) -> Result<Cow<str>>;
    async fn delete_refresh_token(&self, token_id: S) -> Result<()>;
}

impl<S> TokenRepository<S> for Surreal<Any>
where
    S: ToString + Send,
{
    async fn create_refresh_token(
        &self,
        account_id: S,
        expiration_days: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>> {
        self.query(include_str!(
            "../../../resources/queries/middleware/create_refresh_token.surql"
        ))
        .bind(("account_id", account_id.to_string()))
        .bind((
            "expiration_at",
            (Utc::now() + Duration::days(expiration_days)).timestamp(),
        ))
        .bind(("device", device.map(|v| v.to_string())))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(AuthError::TokenCreation.into())
    }

    async fn update_refresh_token(&self, token_id: S, expiration_days: i64) -> Result<Cow<str>> {
        self.query(include_str!(
            "../../../resources/queries/middleware/update_refresh_token.surql"
        ))
        .bind(("token_id", token_id.to_string()))
        .bind((
            "expiration_at",
            (Utc::now() + Duration::days(expiration_days)).timestamp(),
        ))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(AuthError::TokenCreation.into())
    }

    async fn delete_refresh_token(&self, token_id: S) -> Result<()> {
        self.query(include_str!(
            "../../../resources/queries/middleware/delete_refresh_token.surql"
        ))
        .bind(("token_id", token_id.to_string()))
        .await?
        .check()?;

        Ok(())
    }
}

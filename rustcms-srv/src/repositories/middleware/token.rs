use ::chrono::{Duration, Utc};
use ::std::borrow::Cow;

use crate::app::*;

pub trait TokenRepository {
    async fn create_refresh_token(
        &self,
        account_id: impl ToString,
        expiration_days: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>, Error>;
    async fn update_refresh_token(
        &self,
        id: impl ToString,
        expiration_days: i64,
    ) -> Result<Cow<str>, Error>;
    async fn delete_refresh_token(&self, id: impl ToString) -> Result<(), Error>;
}

impl TokenRepository for Database {
    async fn create_refresh_token(
        &self,
        account_id: impl ToString,
        expiration_days: i64,
        device: Option<Cow<'_, str>>,
    ) -> Result<Cow<str>, Error> {
        self.query(include_str!(
            "../../../resources/queries/middleware/create_refresh_token.surql"
        ))
        .bind(("account_id", account_id.to_string()))
        .bind((
            "expiration_at",
            (Utc::now() + Duration::days(expiration_days)).timestamp(),
        ))
        .bind((
            "device",
            device.map_or("web".to_string(), |v| v.to_string()),
        ))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(Error::AuthError(AuthError::TokenCreation))
    }

    async fn update_refresh_token(
        &self,
        id: impl ToString,
        expiration_days: i64,
    ) -> Result<Cow<str>, Error> {
        self.query(include_str!(
            "../../../resources/queries/middleware/update_refresh_token.surql"
        ))
        .bind(("token_id", id.to_string()))
        .bind((
            "expiration_at",
            (Utc::now() + Duration::days(expiration_days)).timestamp(),
        ))
        .await?
        .take::<Option<Cow<str>>>(0)?
        .ok_or(Error::AuthError(AuthError::InvalidToken))
    }

    async fn delete_refresh_token(&self, id: impl ToString) -> Result<(), Error> {
        self.query(include_str!(
            "../../../resources/queries/middleware/delete_refresh_token.surql"
        ))
        .bind(("token_id", id.to_string()))
        .await?
        .check()?;

        Ok(())
    }
}

use ::surrealdb::{Surreal, engine::any::Any};

use crate::{app::*, models::AuthState};

pub trait AccountAuthRepository {
    async fn find_auth_by_credentials<T: AsRef<str>>(
        &self,
        login: T,
        password: T,
    ) -> Result<AuthState<'_>>;
    async fn find_auth_by_token(&self, refresh_token: impl AsRef<str>) -> Result<AuthState<'_>>;
}

impl AccountAuthRepository for Surreal<Any> {
    async fn find_auth_by_credentials<T: AsRef<str>>(
        &self,
        login: T,
        password: T,
    ) -> Result<AuthState<'_>> {
        self.query(include_str!(
            "../../../resources/queries/accounts/find_auth_by_credentials.surql"
        ))
        .bind(("login", login.as_ref().to_string()))
        .bind(("password", password.as_ref().to_string()))
        .await?
        .take::<Option<AuthState>>(0)?
        .ok_or(AuthError::WrongCredentials.into())
    }

    async fn find_auth_by_token(&self, refresh_token: impl AsRef<str>) -> Result<AuthState<'_>> {
        self.query(include_str!(
            "../../../resources/queries/accounts/find_auth_by_refresh_token.surql"
        ))
        .bind(("refresh_token", refresh_token.as_ref().to_string()))
        .await?
        .take::<Option<AuthState>>(0)?
        .ok_or(AuthError::InvalidToken.into())
    }
}

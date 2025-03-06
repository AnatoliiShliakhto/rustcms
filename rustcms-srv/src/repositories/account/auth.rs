use ::surrealdb::{Surreal, engine::any::Any};

use crate::{app::*, models::AuthState};

pub trait AccountAuthRepository<S> {
    async fn find_auth_by_credentials(&self, login: S, password: S) -> Result<AuthState<'_>>;
    async fn find_auth_by_token(&self, refresh_token: S) -> Result<AuthState<'_>>;
}

impl<S> AccountAuthRepository<S> for Surreal<Any>
where
    S: ToString + Send,
{
    async fn find_auth_by_credentials(&self, login: S, password: S) -> Result<AuthState<'_>> {
        self.query(include_str!(
            "../../../resources/queries/accounts/find_auth_by_credentials.surql"
        ))
        .bind(("login", login.to_string()))
        .bind(("password", password.to_string()))
        .await?
        .take::<Option<AuthState>>(0)?
        .ok_or(AuthError::WrongCredentials.into())
    }

    async fn find_auth_by_token(&self, refresh_token: S) -> Result<AuthState<'_>> {
        self.query(include_str!(
            "../../../resources/queries/accounts/find_auth_by_refresh_token.surql"
        ))
        .bind(("refresh_token", refresh_token.to_string()))
        .await?
        .take::<Option<AuthState>>(0)?
        .ok_or(AuthError::TokenExpired.into())
    }
}

use crate::{app::*, services::middleware::*};

pub trait AccountAuthRepository {
    async fn find_auth_by_credentials<T: ToString>(
        &self,
        login: T,
        password: T,
    ) -> Result<AuthState, Error>;
    async fn find_auth_by_token(&self, refresh_token: impl ToString) -> Result<AuthState, Error>;
}

impl AccountAuthRepository for Database {
    async fn find_auth_by_credentials<T: ToString>(
        &self,
        login: T,
        password: T,
    ) -> Result<AuthState, Error> {
        self.query(include_str!(
            "../../../resources/queries/accounts/find_auth_by_credentials.surql"
        ))
        .bind(("login", login.to_string()))
        .bind(("password", password.to_string()))
        .await?
        .take::<Option<AuthState>>(0)?
        .ok_or(Error::AuthError(AuthError::WrongCredentials))
    }

    async fn find_auth_by_token(&self, refresh_token: impl ToString) -> Result<AuthState, Error> {
        self.query(include_str!(
            "../../../resources/queries/accounts/find_auth_by_refresh_token.surql"
        ))
        .bind(("refresh_token", refresh_token.to_string()))
        .await?
        .take::<Option<AuthState>>(0)?
        .ok_or(Error::AuthError(AuthError::InvalidToken))
    }
}

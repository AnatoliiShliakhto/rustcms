use ::surrealdb::{Surreal, engine::any::Any};

use crate::app::*;

pub trait AccountRepository {
    async fn create_account<T: AsRef<str>>(&self, login: T, password: Option<T>) -> Result<()>;
    async fn delete_account(&self, account_id: impl AsRef<str>) -> Result<()>;
}

impl AccountRepository for Surreal<Any> {
    async fn create_account<T: AsRef<str>>(&self, login: T, password: Option<T>) -> Result<()> {
        self.query(include_str!(
            "../../../resources/queries/accounts/create_account.surql"
        ))
        .bind(("login", login.as_ref().to_string()))
        .bind(("password", password.map(|v| v.as_ref().to_string())))
        .await?
        .check()?;

        Ok(())
    }

    async fn delete_account(&self, account_id: impl AsRef<str>) -> Result<()> {
        self.query(include_str!(
            "../../../resources/queries/accounts/delete_account.surql"
        ))
        .bind(("account_id", account_id.as_ref().to_string()))
        .await?
        .check()?;

        Ok(())
    }
}

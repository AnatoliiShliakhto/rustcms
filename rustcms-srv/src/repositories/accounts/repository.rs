use crate::app::*;

pub trait AccountsRepository {
    async fn create_account<T: ToString>(&self, login: T, password: Option<T>)
    -> Result<(), Error>;
    async fn delete_account(&self, id: impl ToString) -> Result<(), Error>;
}

impl AccountsRepository for Database {
    async fn create_account<T: ToString>(
        &self,
        login: T,
        password: Option<T>,
    ) -> Result<(), Error> {
        self.query(include_str!(
            "../../../resources/queries/accounts/create_account.surql"
        ))
        .bind(("login", login.to_string()))
        .bind(("password", password.map(|v| v.to_string())))
        .await?
        .check()?;

        Ok(())
    }

    async fn delete_account(&self, id: impl ToString) -> Result<(), Error> {
        self.query(include_str!(
            "../../../resources/queries/accounts/delete_account.surql"
        ))
        .bind(("account_id", id.to_string()))
        .await?
        .check()?;

        Ok(())
    }
}

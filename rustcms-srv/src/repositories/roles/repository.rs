use crate::{app::*, models::Role};

pub trait RolesRepository {
    async fn roles_list(&self) -> Result<Vec<Role<'static>>, Error>;
}

impl RolesRepository for Database {
    async fn roles_list(&self) -> Result<Vec<Role<'static>>, Error> {
        self.query(include_str!(
            "../../../resources/queries/roles/get_roles.surql"
        ))
            .await?
            .take(0)
            .map(Ok)?
    }
}

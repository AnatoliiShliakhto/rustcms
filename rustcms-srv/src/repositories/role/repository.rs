use ::surrealdb::{Surreal, engine::any::Any};

use crate::{app::*, models::RolePermissions};

pub trait RoleRepository {
    async fn find_roles_with_permissions<'a>(&self) -> Result<Vec<RolePermissions<'a>>>;
}

impl RoleRepository for Surreal<Any> {
    async fn find_roles_with_permissions<'a>(&self) -> Result<Vec<RolePermissions<'a>>> {
        self.query(include_str!(
            "../../../resources/queries/roles/find_roles_with_permissions.surql"
        ))
        .await?
        .take::<Vec<RolePermissions>>(0)
        .map_err(|e| e.into())
    }
}

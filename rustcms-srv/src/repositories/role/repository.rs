use ::surrealdb::{Surreal, engine::any::Any};

use crate::{app::*, models::RolePermissions};

pub trait RoleRepository<S> {
    async fn find_role_permissions(&self, role_id: S) -> Result<RolePermissions>;
}

impl<S> RoleRepository<S> for Surreal<Any>
where
    S: ToString + Send,
{
    async fn find_role_permissions(&self, role_id: S) -> Result<RolePermissions> {
        self.query(include_str!(
            "../../../resources/queries/roles/find_role_permissions.surql"
        ))
            .bind(("login", role_id.to_string()))
            .await?
            .take::<Option<RolePermissions>>(0)?
            .ok_or(DatabaseError::RecordNotFound.into())
    }
}

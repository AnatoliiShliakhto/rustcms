use ::surrealdb::{Surreal, engine::any::Any};

use crate::{app::*, models::RolePermissions};

pub trait CacheRepository {
    async fn create_roles_permissions(&self, roles: Vec<RolePermissions<'static>>) -> Result<()>;
    async fn check_roles_has_permission(
        &self,
        roles: &[impl AsRef<str>],
        permission: impl AsRef<str>,
    ) -> Result<()>;
}

impl CacheRepository for Surreal<Any> {
    async fn create_roles_permissions(&self, roles: Vec<RolePermissions<'static>>) -> Result<()> {
        self.query(include_str!(
            "../../../resources/queries/cache/create_roles_with_permissions.surql"
        ))
        .bind(("roles_with_permissions", roles))
        .await?
        .check()?;

        Ok(())
    }

    async fn check_roles_has_permission(
        &self,
        roles: &[impl AsRef<str>],
        permission: impl AsRef<str>,
    ) -> Result<()> {
        if let Some(true) = self.query(include_str!(
            "../../../resources/queries/cache/check_roles_has_permission.surql"
        ))
        .bind((
            "roles",
            roles
                .iter()
                .map(|v| v.as_ref().to_string())
                .collect::<Vec<String>>(),
        ))
        .bind(("permission", permission.as_ref().to_string()))
        .await?
        .take::<Option<bool>>(0)? {
            Ok(())
        } else { 
            Err(AuthError::AccessForbidden.into())
        }
    }
}

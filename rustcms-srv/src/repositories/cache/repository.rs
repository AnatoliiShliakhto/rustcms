use std::borrow::Cow;

use crate::{app::*, models::RolePermissions};

pub trait CacheRepository {
    async fn import_roles_from_db(&self, db: &Database) -> Result<(), Error>;
    async fn check_roles_has_permission(
        &self,
        roles: &[impl ToString],
        permission: impl ToString,
    ) -> Result<(), Error>;
    async fn get_roles_permissions(&self, roles: &[impl ToString]) -> Result<Vec<Cow<str>>, Error>;
}

impl CacheRepository for Database {
    async fn import_roles_from_db(&self, db: &Database) -> Result<(), Error> {
        let roles = db
            .query(include_str!(
                "../../../resources/queries/cache/import_roles_from_db.surql"
            ))
            .await?
            .take::<Vec<RolePermissions>>(0)?;

        self.query(include_str!(
            "../../../resources/queries/cache/export_roles_to_cache.surql"
        ))
        .bind(("roles_with_permissions", roles))
        .await?
        .check()?;

        Ok(())
    }

    async fn check_roles_has_permission(
        &self,
        roles: &[impl ToString],
        permission: impl ToString,
    ) -> Result<(), Error> {
        if let Some(true) = self
            .query(include_str!(
                "../../../resources/queries/cache/check_roles_has_permission.surql"
            ))
            .bind((
                "roles",
                roles.iter().map(|v| v.to_string()).collect::<Vec<String>>(),
            ))
            .bind(("permission", permission.to_string()))
            .await?
            .take::<Option<bool>>(0)?
        {
            Ok(())
        } else {
            Err(AuthError::AccessForbidden)?
        }
    }

    async fn get_roles_permissions(&self, roles: &[impl ToString]) -> Result<Vec<Cow<str>>, Error> {
        todo!()
    }
}

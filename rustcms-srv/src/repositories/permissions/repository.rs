use crate::{app::*, models::Permission};

pub trait PermissionsRepository {
    async fn permissions_list(&self) -> Result<Vec<Permission<'static>>, Error>;
    async fn create_permission(
        &self,
        name: impl ToString,
        by_id: Option<impl ToString>,
    ) -> Result<Permission<'static>, Error>;
    async fn update_permission(
        &self,
        id: impl ToString,
        name: impl ToString,
        by_id: Option<impl ToString>,
    ) -> Result<Permission<'static>, Error>;
    async fn delete_permission(&self, id: impl ToString) -> Result<(), Error>;
}

impl PermissionsRepository for Database {
    async fn permissions_list(&self) -> Result<Vec<Permission<'static>>, Error> {
        self.query(include_str!(
            "../../../resources/queries/permissions/get_permissions.surql"
        ))
        .await?
        .take(0)
        .map(Ok)?
    }

    async fn create_permission(
        &self,
        name: impl ToString,
        by_id: Option<impl ToString>,
    ) -> Result<Permission<'static>, Error> {
        self.query(include_str!(
            "../../../resources/queries/permissions/create_permission.surql"
        ))
        .bind(("permission_name", name.to_string()))
        .bind(("by_id", by_id.map(|v| v.to_string())))
        .await?
        .take::<Option<Permission>>(0)?
        .ok_or(Error::DatabaseError(DatabaseError::Create))
    }

    async fn update_permission(
        &self,
        id: impl ToString,
        name: impl ToString,
        by_id: Option<impl ToString>,
    ) -> Result<Permission<'static>, Error> {
        self.query(include_str!(
            "../../../resources/queries/permissions/update_permission.surql"
        ))
        .bind(("permission_id", id.to_string()))
        .bind(("permission_name", name.to_string()))
        .bind(("by_id", by_id.map(|v| v.to_string())))
        .await?
        .take::<Option<Permission>>(0)?
        .ok_or(Error::DatabaseError(DatabaseError::Update))
    }

    async fn delete_permission(&self, id: impl ToString) -> Result<(), Error> {
        self.query(include_str!(
            "../../../resources/queries/permissions/delete_permission.surql"
        ))
        .bind(("id", id.to_string()))
        .await?
        .check()?;

        Ok(())
    }
}

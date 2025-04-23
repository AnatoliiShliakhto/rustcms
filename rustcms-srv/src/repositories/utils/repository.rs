use crate::{app::*, models::Metadata};

pub trait UtilsRepository {
    async fn get_metadata(
        &self,
        table: impl ToString,
        id: impl ToString,
    ) -> Result<Metadata, Error>;
}

impl UtilsRepository for Database {
    async fn get_metadata(
        &self,
        table: impl ToString,
        id: impl ToString,
    ) -> Result<Metadata, Error> {
        self.query(include_str!(
            "../../../resources/queries/utils/get_metadata.surql"
        ))
        .bind(("table_name", table.to_string()))
        .bind(("record_id", id.to_string()))
        .await?
        .take::<Option<Metadata>>(0)?
        .ok_or(Error::DatabaseError(DatabaseError::NotFound))
    }
}

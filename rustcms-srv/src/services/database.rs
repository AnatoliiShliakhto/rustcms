use ::surrealdb::opt::auth::Root;
use ::tracing::{info, warn};

use crate::app::*;

pub trait DatabaseExt {
    async fn init(
        &self,
        endpoint: &str,
        namespace: &str,
        name: &str,
        user: &str,
        password: &str,
    ) -> Result<(), Error>;
    async fn database_post_init(&self) -> Result<(), Error>;
    async fn cache_post_init(&self) -> Result<(), Error>;
}

impl DatabaseExt for Database {
    async fn init(
        &self,
        endpoint: &str,
        namespace: &str,
        name: &str,
        username: &str,
        password: &str,
    ) -> Result<(), Error> {
        info!("Initializing database provider...");
        self.connect(endpoint).await?;

        let Some((protocol, _host)) = endpoint.split_once("://") else {
            Err(Error::CustomError("Invalid SurrealDb endpoint"))?
        };

        match protocol {
            "ws" | "wss" | "http" | "https" => {
                self.signin(Root { username, password }).await?;
            }
            "rocksdb" | "file" | "mem" => (),
            _ => Err(Error::CustomError("Unknown SurrealDb endpoint protocol"))?,
        }

        let version = self.version().await?;
        self.use_ns(namespace).use_db(name).await?;
        info!("SurrealDb v{version} '{endpoint}' initialized successfully");

        Ok(())
    }

    async fn database_post_init(&self) -> Result<(), Error> {
        let is_db_initialized = self
            .query(include_str!(
                "../../resources/queries/prelude/check_initial_migration.surql"
            ))
            .await?
            .take::<Option<String>>(0)?
            .is_some();

        if !is_db_initialized {
            let init_result = self
                .query(include_str!(
                    "../../resources/migrations/0000_init.up.surql"
                ))
                .await?
                .take::<Option<bool>>(0)?;
            assert_eq!(init_result, Some(true));

            info!("Migration '0000_init.up.surql' applied");
            warn!("Root account created: login = 'root', password = 'root'")
        } else {
            self.query(include_str!(
                "../../resources/queries/prelude/delete_expired_tokens.surql"
            ))
            .await?
            .check()?;

            info!("Expired tokens deleted")
        }

        Ok(())
    }

    async fn cache_post_init(&self) -> Result<(), Error> {
        self.query(include_str!(
            "../../resources/queries/cache/init_cache.surql"
        ))
        .await?
        .check()?;

        info!("Cache initialized");

        Ok(())
    }
}

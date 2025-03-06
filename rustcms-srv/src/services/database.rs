use ::surrealdb::{Surreal, engine::any::Any, opt::auth::Root};
use ::tracing::{info, warn};

use crate::{app::*, services::config::Config};

pub type Database = Surreal<Any>;

pub trait DatabaseExt {
    async fn init(&self, endpoint: &str, config: &Config) -> Result<()>;
    async fn post_init(&self) -> Result<()>;
}

impl DatabaseExt for Database {
    async fn init(&self, endpoint: &str, config: &Config) -> Result<()> {
        info!("Initializing database provider...");
        self.connect(endpoint).await?;

        let Some((protocol, _host)) = endpoint.split_once("://") else {
            Err(Error::CustomError("Invalid SurrealDb endpoint"))?
        };

        match protocol {
            "ws" | "wss" | "http" | "https" => {
                self.signin(Root {
                    username: &config.db_user,
                    password: &config.db_password,
                })
                .await?;
            }
            "rocksdb" | "file" | "mem" => (),
            _ => Err(Error::CustomError("Unknown SurrealDb endpoint protocol"))?,
        }

        let version = self.version().await?;
        self.use_ns(&*config.db_ns).use_db(&*config.db_name).await?;
        info!("SurrealDb v{version} '{endpoint}' initialized successfully");

        Ok(())
    }

    async fn post_init(&self) -> Result<()> {
        let is_root_account_present = self
            .query(include_str!(
                "../../resources/queries/prelude/find_root_account.surql"
            ))
            .await?
            .take::<Option<bool>>(0)?
            .unwrap_or_default();

        if !is_root_account_present {
            let init_result = self
                .query(include_str!(
                    "../../resources/migrations/0000_init.up.surql"
                ))
                .await?
                .take::<Option<u8>>(0)?;
            assert_eq!(init_result, Some(1u8));

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
}

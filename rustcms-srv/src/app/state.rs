use ::std::sync::LazyLock;
use ::surrealdb::Surreal;
use ::tracing::info;
use ::tracing_appender::non_blocking::WorkerGuard;

use crate::{
    app::*,
    repositories::cache::CacheRepository,
    services::{config::Config, database::DatabaseExt, logger::init_logger},
};

pub struct AppState {
    _logger: WorkerGuard,
    pub config: Config,
    pub database: LazyLock<Database>,
    pub cache: LazyLock<Database>,
}

impl AppState {
    pub async fn init() -> Result<Self, Error> {
        let config = Config::init();
        let _logger = init_logger(config.server.path.join("logs"));

        info!("Server v{} starting...", env!("CARGO_PKG_VERSION"));

        let state = Self {
            _logger,
            config,
            database: LazyLock::new(Surreal::init),
            cache: LazyLock::new(Surreal::init),
        };

        state
            .database
            .init(
                &state.config.database.endpoint,
                &state.config.database.namespace,
                &state.config.database.name,
                &state.config.database.user,
                &state.config.database.password,
            )
            .await?;

        state
            .cache
            .init(
                &state.config.cache.endpoint,
                &state.config.cache.namespace,
                &state.config.cache.name,
                &state.config.cache.user,
                &state.config.cache.password,
            )
            .await?;

        state.database.database_post_init().await?;
        state.cache.cache_post_init().await?;

        state.cache.import_roles_from_db(&state.database).await?;

        Ok(state)
    }
}

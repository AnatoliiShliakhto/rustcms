use ::std::sync::LazyLock;
use ::surrealdb::Surreal;
use ::tracing::info;
use ::tracing_appender::non_blocking::WorkerGuard;

use crate::{
    app::*,
    repositories::{cache::CacheRepository, role::RoleRepository},
    services::{
        config::Config,
        database::{Database, DatabaseExt},
        logger::init_logger,
    },
};

pub struct AppState {
    _logger: WorkerGuard,
    pub cfg: Config,
    pub db: LazyLock<Database>,
    pub cache: LazyLock<Database>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let cfg = Config::init();
        let _logger = init_logger(cfg.path.join("logs"));

        info!("Server v{} starting...", env!("CARGO_PKG_VERSION"));

        let state = Self {
            _logger,
            cfg,
            db: LazyLock::new(Surreal::init),
            cache: LazyLock::new(Surreal::init),
        };

        state.db.init(&state.cfg.db_endpoint, &state.cfg).await?;
        state.cache.init("mem://", &state.cfg).await?;

        state.db.database_post_init().await?;
        state.cache.cache_post_init().await?;
        
        let roles = state.db.find_roles_with_permissions().await?;
        state.cache.create_roles_permissions(roles).await?;

        Ok(state)
    }
}

use ::std::sync::LazyLock;
use ::surrealdb::Surreal;
use ::tracing::info;
use ::tracing_appender::non_blocking::WorkerGuard;

use crate::{
    app::*,
    services::{
        config::Config,
        database::{Database, DatabaseExt},
        logger::init_logger,
        middleware::JwtKeys,
    },
};

pub struct AppState {
    _logger: WorkerGuard,
    pub cfg: Config,
    pub keys: JwtKeys,
    pub db: LazyLock<Database>,
    pub cache: LazyLock<Database>,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let cfg = Config::init();
        let _logger = init_logger(cfg.path.join("logs"));
        let keys = JwtKeys::init(cfg.jwt_secret.as_bytes());

        info!("Server v{} starting...", env!("CARGO_PKG_VERSION"));

        let state = Self {
            _logger,
            cfg,
            keys,
            db: LazyLock::new(Surreal::init),
            cache: LazyLock::new(Surreal::init),
        };

        state.db.init(&state.cfg.db_endpoint, &state.cfg).await?;
        state.cache.init("mem://", &state.cfg).await?;

        state.db.post_init().await?;

        Ok(state)
    }
}

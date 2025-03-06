use ::std::path::PathBuf;
use ::tracing_appender::{
    non_blocking::WorkerGuard,
    rolling::{RollingFileAppender, Rotation},
};
use ::tracing_subscriber::{
    filter::LevelFilter, fmt::layer, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

pub fn init_logger(path: PathBuf) -> WorkerGuard {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .filename_prefix("srv")
        .filename_suffix("log")
        .max_log_files(10)
        .build(path)
        .expect("failed to initialize rolling file appender");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let stdout_layer = layer().compact();
    let store_layer = layer().json().with_writer(non_blocking);

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(store_layer)
        .with(env_filter)
        .init();

    guard
}

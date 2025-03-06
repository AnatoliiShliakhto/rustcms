use ::std::{borrow::Cow, env::current_dir, net::Ipv4Addr, path::PathBuf};

pub struct Config {
    pub path: PathBuf,
    pub host: Ipv4Addr,
    pub http_port: u16,
    pub https_port: u16,
    pub jwt_secret: Cow<'static, str>,
    pub db_endpoint: Cow<'static, str>,
    pub db_ns: Cow<'static, str>,
    pub db_name: Cow<'static, str>,
    pub db_user: Cow<'static, str>,
    pub db_password: Cow<'static, str>,
    pub jwt_issuer: Cow<'static, str>,
    pub jwt_subject: Cow<'static, str>,
    pub jwt_access_expiration: i64,
    pub jwt_refresh_expiration: i64,
}

impl Config {
    pub fn init() -> Self {
        Self {
            path: current_dir().unwrap().join("publish"),
            host: get_env("RUSTCMS_HOST", "0.0.0.0")
                .parse()
                .unwrap_or(Ipv4Addr::new(0, 0, 0, 0)),
            http_port: get_env("RUSTCMS_HTTP_PORT", "80").parse().unwrap_or(80),
            https_port: get_env("RUSTCMS_HTTPS_PORT", "443").parse().unwrap_or(443),
            jwt_secret: get_env("RUSTCMS_JWT_SECRET", "secret"),
            db_endpoint: get_env("RUSTCMS_DATABASE_ENDPOINT", "rocksdb://./publish/db"),
            db_ns: get_env("RUSTCMS_DATABASE_NS", "novacms"),
            db_name: get_env("RUSTCMS_DATABASE_NAME", "core"),
            db_user: get_env("RUSTCMS_DATABASE_USER", "root"),
            db_password: get_env("RUSTCMS_DATABASE_PASSWORD", "root"),
            jwt_issuer: get_env("RUSTCMS_JWT_ISSUER", "novacms"),
            jwt_subject: get_env("RUSTCMS_JWT_SUBJECT", "core"),
            jwt_access_expiration: get_env("RUSTCMS_JWT_ACCESS_EXPIRATION", "10")
                .parse()
                .unwrap_or(10),
            jwt_refresh_expiration: get_env("RUSTCMS_JWT_REFRESH_EXPIRATION", "15")
                .parse()
                .unwrap_or(15),
        }
    }
}

fn get_env<'a>(name: &str, default: &'a str) -> Cow<'a, str> {
    dotenv::dotenv().ok();

    if let Ok(value) = dotenv::var(name) {
        Cow::Owned(value)
    } else {
        Cow::Borrowed(default)
    }
}

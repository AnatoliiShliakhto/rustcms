use ::axum::http::{HeaderName, HeaderValue};
use ::axum_extra::headers::HeaderMap;
use ::serde::{Deserialize, Deserializer};
use ::std::str::FromStr;
use ::std::{borrow::Cow, fs::File, net::Ipv4Addr, path::PathBuf};

use crate::services::middleware::jwt_keys::JwtKeys;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub url: Cow<'static, str>,
    pub host: Ipv4Addr,
    pub http_port: u16,
    pub https_port: u16,
    pub path: PathBuf,
}

pub struct JwtConfig {
    pub keys: JwtKeys,
    pub issuer: Cow<'static, str>,
    pub subject: Cow<'static, str>,
    pub access_expiration: i64,
    pub refresh_expiration: i64,
}

#[derive(Deserialize)]
pub struct SecurityConfig {
    #[serde(deserialize_with = "deserialize_jwt_config")]
    pub jwt: JwtConfig,
    pub set_cookie: Cow<'static, str>,
    pub max_body_limit: usize,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub endpoint: Cow<'static, str>,
    pub namespace: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub user: Cow<'static, str>,
    pub password: Cow<'static, str>,
}

#[derive(Deserialize)]
pub struct CacheConfig {
    pub endpoint: Cow<'static, str>,
    pub namespace: Cow<'static, str>,
    pub name: Cow<'static, str>,
    pub user: Cow<'static, str>,
    pub password: Cow<'static, str>,
}

#[derive(Clone, Deserialize)]
struct StaticHeader<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
}

#[derive(Deserialize)]
pub struct HeadersConfig {
    #[serde(deserialize_with = "deserialize_header_map")]
    pub common: HeaderMap,
    #[serde(deserialize_with = "deserialize_header_map")]
    pub api: HeaderMap,
    #[serde(deserialize_with = "deserialize_header_map")]
    pub public_storage: HeaderMap,
    #[serde(deserialize_with = "deserialize_header_map")]
    pub private_storage: HeaderMap,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub security: SecurityConfig,
    pub headers: HeadersConfig,
}

impl Config {
    pub fn init() -> Self {
        dotenv::dotenv().ok();

        let config_path = PathBuf::from(&*get_env(
            "RUSTCMS_SERVER_CONFIG_PATH",
            "server-config.json",
        ));

        if config_path.exists() {
            println!(
                "Server configuration: configuration file {:?} founded.",
                config_path
            );
            Self::init_json(config_path)
        } else {
            println!(
                "Server configuration: configuration file {:?} not founded.",
                config_path
            );
            Self::init_env()
        }
    }

    fn init_env() -> Self {
        println!("Server configuration: using environment variables...");

        let jwt_keys = JwtKeys::init(get_env("RUSTCMS_JWT_SECRET", "secret").as_bytes());

        Self {
            server: ServerConfig {
                path: PathBuf::from(&*get_env("RUSTCMS_DATA_PATH", "./publish/data")),
                url: get_env("RUSTCMS_URL", "https://localhost"),
                host: get_env("RUSTCMS_HOST", "0.0.0.0")
                    .parse()
                    .unwrap_or(Ipv4Addr::new(0, 0, 0, 0)),
                http_port: get_env("RUSTCMS_HTTP_PORT", "80").parse().unwrap_or(80),
                https_port: get_env("RUSTCMS_HTTPS_PORT", "443").parse().unwrap_or(443),
            },
            database: DatabaseConfig {
                endpoint: get_env("RUSTCMS_DATABASE_ENDPOINT", "mem://"),
                namespace: get_env("RUSTCMS_DATABASE_NAMESPACE", "rustcms"),
                name: get_env("RUSTCMS_DATABASE_NAME", "core"),
                user: get_env("RUSTCMS_DATABASE_USER", "root"),
                password: get_env("RUSTCMS_DATABASE_PASSWORD", "root"),
            },
            cache: CacheConfig {
                endpoint: get_env("RUSTCMS_CACHE_ENDPOINT", "mem://"),
                namespace: get_env("RUSTCMS_CACHE_NAMESPACE", "rustcms"),
                name: get_env("RUSTCMS_CACHE_NAME", "cache"),
                user: get_env("RUSTCMS_CACHE_USER", "root"),
                password: get_env("RUSTCMS_CACHE_PASSWORD", "root"),
            },
            security: SecurityConfig {
                jwt: JwtConfig {
                    keys: jwt_keys,
                    issuer: get_env("RUSTCMS_JWT_ISSUER", "https:://localhost"),
                    subject: get_env("RUSTCMS_JWT_SUBJECT", "https://localhost/api"),
                    access_expiration: get_env("RUSTCMS_JWT_ACCESS_EXPIRATION", "10")
                        .parse()
                        .unwrap_or(10),
                    refresh_expiration: get_env("RUSTCMS_JWT_ACCESS_EXPIRATION", "10")
                        .parse()
                        .unwrap_or(10),
                },
                set_cookie: get_env("RUSTCMS_SET_COOKIE_HEADER", "HttpOnly;"),
                max_body_limit: get_env("RUSTCMS_MAX_BODY_LIMIT", "104857600")
                    .parse()
                    .unwrap_or(104_857_600),
            },
            headers: HeadersConfig {
                common: get_env_headers("RUSTCMS_COMMON_RESPONSE_HEADER"),
                api: get_env_headers("RUSTCMS_API_RESPONSE_HEADER"),
                public_storage: get_env_headers("RUSTCMS_PUBLIC_STORAGE_RESPONSE_HEADER"),
                private_storage: get_env_headers("RUSTCMS_PRIVATE_STORAGE_RESPONSE_HEADER"),
            },
        }
    }

    fn init_json(config_file: PathBuf) -> Self {
        println!("Server configuration: using configuration file...");
        serde_json::from_reader(File::open(config_file).unwrap()).unwrap()
    }
}

fn get_env<'a>(name: &str, default: &'a str) -> Cow<'a, str> {
    if let Ok(value) = dotenv::var(name) {
        Cow::Owned(value)
    } else {
        Cow::Borrowed(default)
    }
}

fn get_env_headers(key: &str) -> HeaderMap {
    let mut response_headers = HeaderMap::new();
    let mut header_cursor = 1;

    while let Ok(value) = dotenv::var(format!("{key}_{header_cursor}")) {
        let (header_name, header_value) = value.split_once(":").unwrap();
        response_headers.insert(
            HeaderName::from_str(header_name.trim()).unwrap(),
            HeaderValue::from_str(header_value.trim()).unwrap(),
        );
        header_cursor += 1
    }

    response_headers
}

fn deserialize_jwt_config<'de, D>(deserializer: D) -> Result<JwtConfig, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    pub struct JwtVisitor {
        pub secret: Cow<'static, str>,
        pub issuer: Cow<'static, str>,
        pub subject: Cow<'static, str>,
        pub access_expiration: i64,
        pub refresh_expiration: i64,
    }

    let jwt_visitor = <JwtVisitor>::deserialize(deserializer)?;

    Ok(JwtConfig {
        keys: JwtKeys::init(jwt_visitor.secret.as_bytes()),
        issuer: jwt_visitor.issuer,
        subject: jwt_visitor.subject,
        access_expiration: jwt_visitor.access_expiration,
        refresh_expiration: jwt_visitor.refresh_expiration,
    })
}

fn deserialize_header_map<'de, D>(deserializer: D) -> Result<HeaderMap, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct HeaderVisitor {
        name: String,
        value: String,
    }

    let header_map_visitor = <Vec<HeaderVisitor>>::deserialize(deserializer)?;

    let mut header_map = HeaderMap::new();
    for header in header_map_visitor.iter() {
        header_map.insert(
            HeaderName::from_str(&header.name).unwrap(),
            HeaderValue::from_str(&header.value).unwrap(),
        );
    }

    Ok(header_map)
}

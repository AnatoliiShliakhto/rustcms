use ::axum::http::{HeaderName, HeaderValue};
use ::axum_extra::headers::HeaderMap;
use ::serde::Deserialize;
use ::serde_json::Value;
use ::std::str::FromStr;
use ::std::{borrow::Cow, fs::File, net::Ipv4Addr, path::PathBuf};

use crate::services::middleware::JwtKeys;

pub struct Config {
    pub jwt_keys: JwtKeys,
    pub path: PathBuf,
    pub url: Cow<'static, str>,
    pub host: Ipv4Addr,
    pub http_port: u16,
    pub https_port: u16,
    pub max_body_limit: usize,
    pub db_endpoint: Cow<'static, str>,
    pub db_ns: Cow<'static, str>,
    pub db_name: Cow<'static, str>,
    pub db_user: Cow<'static, str>,
    pub db_password: Cow<'static, str>,
    pub jwt_issuer: Cow<'static, str>,
    pub jwt_subject: Cow<'static, str>,
    pub jwt_access_expiration: i64,
    pub jwt_refresh_expiration: i64,
    pub static_response_headers: HeaderMap,
    pub public_storage_response_headers: HeaderMap,
    pub private_storage_response_headers: HeaderMap,
    pub api_response_headers: HeaderMap,
}

#[derive(Clone, Deserialize)]
struct StaticHeader<'a> {
    pub name: Cow<'a, str>,
    pub value: Cow<'a, str>,
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
            jwt_keys,
            path: PathBuf::from(&*get_env("RUSTCMS_DATA_PATH", "./publish/data")),
            url: get_env("RUSTCMS_URL", "https://localhost"),
            host: get_env("RUSTCMS_HOST", "0.0.0.0")
                .parse()
                .unwrap_or(Ipv4Addr::new(0, 0, 0, 0)),
            http_port: get_env("RUSTCMS_HTTP_PORT", "80").parse().unwrap_or(80),
            https_port: get_env("RUSTCMS_HTTPS_PORT", "443").parse().unwrap_or(443),
            db_endpoint: get_env("RUSTCMS_DATABASE_ENDPOINT", "mem://"),
            db_ns: get_env("RUSTCMS_DATABASE_NS", "rustcms"),
            db_name: get_env("RUSTCMS_DATABASE_NAME", "core"),
            db_user: get_env("RUSTCMS_DATABASE_USER", "root"),
            db_password: get_env("RUSTCMS_DATABASE_PASSWORD", "root"),
            jwt_issuer: get_env("RUSTCMS_JWT_ISSUER", "https:://localhost"),
            jwt_subject: get_env("RUSTCMS_JWT_SUBJECT", "https://localhost/api"),
            jwt_access_expiration: get_env("RUSTCMS_JWT_ACCESS_EXPIRATION", "10")
                .parse()
                .unwrap_or(10),
            jwt_refresh_expiration: get_env("RUSTCMS_JWT_REFRESH_EXPIRATION", "15")
                .parse()
                .unwrap_or(15),
            max_body_limit: get_env("RUSTCMS_MAX_BODY_LIMIT", "104857600")
                .parse()
                .unwrap_or(104_857_600),
            static_response_headers: get_env_headers("RUSTCMS_STATIC_RESPONSE_HEADER"),
            public_storage_response_headers: get_env_headers(
                "RUSTCMS_PUBLIC_STORAGE_RESPONSE_HEADER",
            ),
            private_storage_response_headers: get_env_headers(
                "RUSTCMS_PRIVATE_STORAGE_RESPONSE_HEADER",
            ),
            api_response_headers: get_env_headers("RUSTCMS_API_RESPONSE_HEADER"),
        }
    }

    fn init_json(config_file: PathBuf) -> Self {
        println!("Server configuration: using configuration file...");

        let config: Value = serde_json::from_reader(File::open(config_file).unwrap()).unwrap();
        let jwt_keys = JwtKeys::init(config["jwt"]["secret"].as_str().unwrap().as_bytes());

        Self {
            jwt_keys,
            path: PathBuf::from(config["server"]["path"].as_str().unwrap()),
            url: Cow::Owned(config["server"]["url"].as_str().unwrap().to_owned()),
            host: config["server"]["host"].as_str().unwrap().parse().unwrap(),
            http_port: config["server"]["http_port"].as_u64().unwrap() as u16,
            https_port: config["server"]["https_port"].as_u64().unwrap() as u16,
            max_body_limit: config["server"]["max_body_limit"].as_u64().unwrap() as usize,
            db_endpoint: Cow::Owned(config["database"]["endpoint"].as_str().unwrap().to_owned()),
            db_ns: Cow::Owned(config["database"]["namespace"].as_str().unwrap().to_owned()),
            db_name: Cow::Owned(config["database"]["database"].as_str().unwrap().to_owned()),
            db_user: Cow::Owned(config["database"]["user"].as_str().unwrap().to_owned()),
            db_password: Cow::Owned(config["database"]["password"].as_str().unwrap().to_owned()),
            jwt_issuer: Cow::Owned(config["jwt"]["issuer"].as_str().unwrap().to_owned()),
            jwt_subject: Cow::Owned(config["jwt"]["subject"].as_str().unwrap().to_owned()),
            jwt_access_expiration: config["jwt"]["access_expiration_minutes"].as_i64().unwrap(),
            jwt_refresh_expiration: config["jwt"]["refresh_expiration_days"].as_i64().unwrap(),
            static_response_headers: get_json_headers(config["static_response_headers"].clone()),
            public_storage_response_headers: get_json_headers(
                config["public_storage_response_headers"].clone(),
            ),
            private_storage_response_headers: get_json_headers(
                config["private_storage_response_headers"].clone(),
            ),
            api_response_headers: get_json_headers(config["api_response_headers"].clone()),
        }
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

fn get_json_headers(json_value: Value) -> HeaderMap {
    let mut response_headers = HeaderMap::new();
    for header in serde_json::from_value::<Vec<StaticHeader>>(json_value)
        .unwrap()
        .iter()
    {
        response_headers.insert(
            HeaderName::from_str(&header.name).unwrap(),
            HeaderValue::from_str(&header.value).unwrap(),
        );
    }

    response_headers
}

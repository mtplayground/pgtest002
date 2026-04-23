use std::{env, error::Error, fmt, net::SocketAddr, num::ParseIntError};

use axum::extract::FromRef;
use leptos::config::{LeptosOptions, get_configuration};

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub leptos_site_addr: SocketAddr,
    pub leptos_site_root: String,
    leptos_options: LeptosOptions,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        let host = read_env("HOST")?;
        let port = read_env("PORT")?.parse().map_err(ConfigError::InvalidPort)?;
        let database_url = read_env("DATABASE_URL")?;
        let leptos_site_addr = read_env("LEPTOS_SITE_ADDR")?
            .parse()
            .map_err(ConfigError::InvalidSocketAddr)?;
        let leptos_site_root = read_env("LEPTOS_SITE_ROOT")?;
        let mut leptos_options = get_configuration(Some("Cargo.toml"))
            .map_err(|error| ConfigError::Leptos(error.to_string()))?
            .leptos_options;

        leptos_options.site_addr = leptos_site_addr;
        leptos_options.site_root = leptos_site_root.clone().into();

        Ok(Self {
            host,
            port,
            database_url,
            leptos_site_addr,
            leptos_site_root,
            leptos_options,
        })
    }

    pub fn listen_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl FromRef<Config> for LeptosOptions {
    fn from_ref(config: &Config) -> Self {
        config.leptos_options.clone()
    }
}

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(&'static str),
    InvalidPort(ParseIntError),
    InvalidSocketAddr(std::net::AddrParseError),
    Leptos(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnv(name) => write!(f, "missing required environment variable {name}"),
            Self::InvalidPort(error) => write!(f, "invalid PORT value: {error}"),
            Self::InvalidSocketAddr(error) => {
                write!(f, "invalid LEPTOS_SITE_ADDR value: {error}")
            }
            Self::Leptos(error) => write!(f, "failed to load Leptos configuration: {error}"),
        }
    }
}

impl Error for ConfigError {}

fn read_env(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingEnv(name))
}

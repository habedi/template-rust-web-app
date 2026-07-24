use axum::http::{HeaderValue, Method, header};
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    /// Comma-separated list of allowed origins, or `*` to allow any origin.
    #[serde(default = "default_cors_origins")]
    pub cors_origins: Vec<String>,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_log_pretty")]
    pub log_pretty: bool,
    pub database_url: String,
    #[serde(default = "default_database_max_connections")]
    pub database_max_connections: u32,
    /// How long to wait for a connection before giving up. It bounds the
    /// startup migration step, so an unreachable database fails quickly.
    #[serde(default = "default_database_acquire_timeout_seconds")]
    pub database_acquire_timeout_seconds: u64,
}

impl Config {
    /// Reads the configuration from the process environment.
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<Config>()
    }

    /// Builds the connection pool without waiting for the database to accept a
    /// connection, so startup does not depend on the database being ready yet.
    pub fn get_pg_pool(&self) -> Result<PgPool, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(self.database_max_connections)
            .acquire_timeout(Duration::from_secs(self.database_acquire_timeout_seconds))
            .connect_lazy(&self.database_url)
    }

    pub fn get_cors_layer(&self) -> CorsLayer {
        let cors = CorsLayer::new()
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

        match self.cors_origins.iter().any(|origin| origin == "*") {
            true => cors.allow_origin(tower_http::cors::Any),
            false => {
                let origins: Vec<HeaderValue> = self
                    .cors_origins
                    .iter()
                    .filter_map(|origin| origin.parse::<HeaderValue>().ok())
                    .collect();
                cors.allow_origin(origins)
            }
        }
    }
}

fn default_port() -> u16 {
    8080
}

fn default_cors_origins() -> Vec<String> {
    vec![]
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_pretty() -> bool {
    false
}

fn default_database_max_connections() -> u32 {
    5
}

fn default_database_acquire_timeout_seconds() -> u64 {
    10
}

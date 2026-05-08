// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # WhailMail Common — Shared Domain Layer (´・ω・`)
//!
//! Our single import point for everything WhailMail needs across crates.
//!
//! This is the heart of our type system: domain models, errors, DTOs, config,
//! and constants all live here. No business logic, no I/O — just the vocabulary
//! we use to talk about users, accounts, emails, and filters.
//!
//! **Use this to:** Import core types in `whailmail-db`, `whailmail-api`, etc.
//!
//! ```no_run
//! use whailmail_common::{EAppError, RAppResult, SEmail, SUser};
//! ```

pub mod config;
pub mod constants;
pub mod dto;
pub mod error;
pub mod macros;
pub mod theme;
pub mod types;

// Re-export key types for convenience
use tracing_subscriber::{filter::EnvFilter, fmt, prelude::*};
pub use {
    config::SConfig,
    error::{EAppError, RAppResult},
    types::*
};

/// Initialize tracing/logging infrastructure
/// Call this once at app startup

pub fn init_tracing()
{
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("mailserver=debug,info"));

    if cfg!(debug_assertions)
    {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .pretty()
                    .with_target(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
            )
            .init();
    }
    else
    {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt::layer().json())
            .init();
    }

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "Mailserver initialized"
    );
}

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig
{
    pub database_url:  String,
    pub jwt_secret:    String,
    pub imap_host:     String,
    pub imap_port:     u16,
    pub smtp_host:     String,
    pub smtp_port:     u16,
    pub max_mail_size: u64,
    pub index_path:    String
}

impl AppConfig
{
    /// Load config from environment variables
    pub fn from_env() -> RAppResult<Self>
    {
        Ok(Self {
            database_url:  std::env::var("DATABASE_URL").map_err(|_| {
                EAppError::ConfigError(anyhow::anyhow!("DATABASE_URL not set"))
            })?,
            jwt_secret:    std::env::var("JWT_SECRET").map_err(|_| {
                EAppError::ConfigError(anyhow::anyhow!("JWT_SECRET not set"))
            })?,
            imap_host:     std::env::var("IMAP_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            imap_port:     std::env::var("IMAP_PORT")
                .unwrap_or_else(|_| "143".to_string())
                .parse()
                .map_err(|_| {
                    EAppError::ConfigError(anyhow::anyhow!(
                        "IMAP_PORT must be numeric"
                    ))
                })?,
            smtp_host:     std::env::var("SMTP_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            smtp_port:     std::env::var("SMTP_PORT")
                .unwrap_or_else(|_| "25".to_string())
                .parse()
                .map_err(|_| {
                    EAppError::ConfigError(anyhow::anyhow!(
                        "SMTP_PORT must be numeric"
                    ))
                })?,
            max_mail_size: std::env::var("MAX_MAIL_SIZE")
                .unwrap_or_else(|_| "52428800".to_string())
                .parse()
                .map_err(|_| {
                    EAppError::ConfigError(anyhow::anyhow!(
                        "MAX_MAIL_SIZE must be numeric"
                    ))
                })?,
            index_path:    std::env::var("INDEX_PATH")
                .unwrap_or_else(|_| "./data/index".to_string())
        })
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    // Test-specific config builder
    struct TestEnv
    {
        db_url:     String,
        jwt_secret: String
    }

    impl TestEnv
    {
        fn new() -> Self
        {
            Self {
                db_url:     format!(
                    "postgresql://localhost/test_{}",
                    uuid::Uuid::new_v4()
                ),
                jwt_secret: "test-secret-key".to_string()
            }
        }

        fn config(&self) -> AppConfig
        {
            AppConfig {
                database_url:  self.db_url.clone(),
                jwt_secret:    self.jwt_secret.clone(),
                imap_host:     "localhost".to_string(),
                imap_port:     143,
                smtp_host:     "localhost".to_string(),
                smtp_port:     25,
                max_mail_size: 52428800,
                index_path:    format!("./test_index_{}", uuid::Uuid::new_v4())
            }
        }
    }

    #[test]
    fn test_config_creation()
    {
        let env = TestEnv::new();
        let config = env.config();

        assert!(!config.database_url.is_empty());
        assert!(!config.jwt_secret.is_empty());
    }
}

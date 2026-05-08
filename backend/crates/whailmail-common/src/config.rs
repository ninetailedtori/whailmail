// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # Configuration Structures — The Knobs & Dials (´｀)
//!
//! All the tunable stuff: database URLs, JWT secrets, log levels, mail server
//! defaults, connection pool sizes. This is where environment-specific
//! behaviour lives.
//!
//! **Organized by concern:**
//! - App config (port, JWT secret, log level, environment)
//! - Database config (connection pool, driver choice)
//! - Mail server defaults (IMAP/SMTP ports, timeouts, TLS settings)
//! - Feature flags (self-hosted mode, encryption, etc.)
//!
//! Meant to be loaded from `.env` or environment variables at startup,
//! not hardcoded.

use {
    serde::{Deserialize, Serialize},
    std::{env, path::PathBuf}
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SConfig
{
    pub db:          SDbConfig,
    pub jwt:         SJwtConfig,
    pub server:      SServerConfig,
    pub imap_sync:   SImapConfig,
    pub smtp:        SSmtpConfig,
    pub mail_limits: SMailLimitsConfig,
    #[cfg(feature = "mailserver")]
    pub features:    SFeaturesConfig
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDbConfig
{
    pub url:                     String,
    pub max_connections:         u32,
    pub min_idle:                u32,
    pub connection_timeout_secs: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJwtConfig
{
    pub secret:                  String,
    pub expiration_secs:         u64,
    pub refresh_expiration_secs: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServerConfig
{
    pub host:                 String,
    pub port:                 u16,
    pub log_level:            String,
    pub cors_allowed_origins: Vec<String>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SImapConfig
{
    pub poll_interval_secs:      u64,
    pub max_concurrent_syncs:    u32,
    pub connection_timeout_secs: u64,
    pub idle_timeout_secs:       u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSmtpConfig
{
    pub connection_timeout_secs: u64,
    pub send_timeout_secs:       u64,
    pub max_retries:             u32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SMailLimitsConfig
{
    pub max_attachment_size_bytes: u64,
    pub max_email_body_size_bytes: u64,
    pub max_recipients_per_email:  u32
}

#[cfg(feature = "mailserver")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFeaturesConfig
{
    pub self_hosted_mode:   bool,
    pub encryption_enabled: bool,
    pub enable_s3_storage:  bool,
    #[serde(default)]
    pub s3_config:          Option<SFeatureS3>,
    #[serde(default)]
    pub encryption_config:  Option<SFeatureEncryption>
}

#[cfg(feature = "mailserver")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFeatureS3
{
    pub bucket:   String,
    pub region:   String,
    pub endpoint: Option<String>
}

#[cfg(feature = "mailserver")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFeatureEncryption
{
    pub algorithm:         String,
    pub key_rotation_days: u32
}

// Helpers

fn config_file() -> PathBuf
{
    std::fs::canonicalize("whailmail.toml").expect("config file not found")
}

fn get_env<T>(name: &str, default: T) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr + Clone,
    T::Err: std::error::Error + 'static
{
    env::var(name)
        .ok()
        .map(|s| s.parse::<T>())
        .transpose()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
        .map(|opt| opt.unwrap_or(default))
}

fn load_toml() -> Result<SConfig, Box<dyn std::error::Error>>
{
    let path = config_file();
    let contents = std::fs::read_to_string(path)?;
    toml::from_str::<SConfig>(&contents)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

// Config & Validation

impl SConfig
{
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        dotenv::dotenv().ok();

        let mut config = load_toml()?;

        config.jwt.secret = env::var("JWT_SECRET").unwrap_or(config.jwt.secret);
        config.server.host = get_env("SERVER_HOST", config.server.host)?;
        config.server.port = get_env("SERVER_PORT", config.server.port)?;
        config.server.log_level =
            get_env("LOG_LEVEL", config.server.log_level)?;

        if let Ok(origins) = env::var("CORS_ALLOWED_ORIGINS")
        {
            config.server.cors_allowed_origins =
                origins.split(',').map(|s| s.trim().to_string()).collect();
        }

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        self.db.validate()?;
        self.jwt.validate()?;
        self.server.validate()?;
        #[cfg(feature = "mailserver")]
        {
            self.imap_sync.validate()?;
            self.smtp.validate()?;
            self.features.validate()?;
        }
        self.mail_limits.validate()?;
        Ok(())
    }
}

impl SDbConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if !self.url.starts_with("sqlite://")
            && !self.url.starts_with("postgres://")
        {
            return Err(
                "DB_URL must start with sqlite:// or postgres://".into()
            );
        }
        if self.max_connections == 0 || self.max_connections > 100
        {
            return Err("DB_MAX_CONNECTIONS must be in [1, 100]".into());
        }
        if self.min_idle > self.max_connections
        {
            return Err("DB_MIN_IDLE must be <= DB_MAX_CONNECTIONS".into());
        }
        if self.connection_timeout_secs == 0
            || self.connection_timeout_secs > 300
        {
            return Err("DB_CONNECTION_TIMEOUT_SECS must be in [1, 300]".into());
        }
        Ok(())
    }
}

impl SJwtConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.secret.len() < 32
        {
            return Err("JWT_SECRET must be at least 32 characters".into());
        }
        if self.expiration_secs == 0
        {
            return Err("JWT_EXPIRATION_SECS must be > 0".into());
        }
        if self.refresh_expiration_secs < self.expiration_secs
        {
            return Err("JWT_REFRESH_EXPIRATION_SECS must be >= \
                        JWT_EXPIRATION_SECS"
                .into());
        }
        if self.expiration_secs < 300
        {
            eprintln!(
                "WARNING: JWT_EXPIRATION_SECS is very short ({} secs). \
                 Consider >= 3600.",
                self.expiration_secs
            );
        }
        Ok(())
    }
}

impl SServerConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.host.is_empty()
        {
            return Err("SERVER_HOST cannot be empty".into());
        }
        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&self.log_level.as_str())
        {
            return Err(format!(
                "LOG_LEVEL must be one of: {:?}",
                valid_log_levels
            )
            .into());
        }
        if self.cors_allowed_origins.is_empty()
        {
            return Err("CORS_ALLOWED_ORIGINS cannot be empty".into());
        }
        for origin in &self.cors_allowed_origins
        {
            if !origin.starts_with("http://") && !origin.starts_with("https://")
            {
                return Err(format!(
                    "CORS origin '{}' must start with http:// or https://",
                    origin
                )
                .into());
            }
        }
        Ok(())
    }
}

impl SImapConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.poll_interval_secs == 0
        {
            return Err("IMAP_POLL_INTERVAL_SECS must be > 0".into());
        }
        if self.max_concurrent_syncs == 0 || self.max_concurrent_syncs > 50
        {
            return Err("IMAP_MAX_CONCURRENT_SYNCS must be in [1, 50]".into());
        }
        if self.connection_timeout_secs == 0
            || self.connection_timeout_secs > 300
        {
            return Err(
                "IMAP_CONNECTION_TIMEOUT_SECS must be in [1, 300]".into()
            );
        }
        if self.idle_timeout_secs < self.connection_timeout_secs
        {
            return Err("IMAP_IDLE_TIMEOUT_SECS must be >= \
                        IMAP_CONNECTION_TIMEOUT_SECS"
                .into());
        }
        if self.poll_interval_secs < 60
        {
            eprintln!(
                "WARNING: IMAP_POLL_INTERVAL_SECS < 60s may cause excessive \
                 server load"
            );
        }
        Ok(())
    }
}

impl SSmtpConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.connection_timeout_secs == 0
            || self.connection_timeout_secs > 300
        {
            return Err(
                "SMTP_CONNECTION_TIMEOUT_SECS must be in [1, 300]".into()
            );
        }
        if self.send_timeout_secs == 0 || self.send_timeout_secs > 600
        {
            return Err("SMTP_SEND_TIMEOUT_SECS must be in [1, 600]".into());
        }
        if self.max_retries > 10
        {
            return Err("SMTP_MAX_RETRIES must be in [0, 10]".into());
        }
        Ok(())
    }
}

impl SMailLimitsConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.max_attachment_size_bytes == 0
            || self.max_attachment_size_bytes > 500_000_000
        {
            return Err("MAX_ATTACHMENT_SIZE_BYTES must be in [1, \
                        500_000_000]"
                .into());
        }
        if self.max_email_body_size_bytes == 0
            || self.max_email_body_size_bytes > 1_000_000_000
        {
            return Err("MAX_EMAIL_BODY_SIZE_BYTES must be in [1, \
                        1_000_000_000]"
                .into());
        }
        if self.max_recipients_per_email == 0
            || self.max_recipients_per_email > 1000
        {
            return Err("MAX_RECIPIENTS_PER_EMAIL must be in [1, 1000]".into());
        }
        if self.max_email_body_size_bytes < self.max_attachment_size_bytes
        {
            return Err("MAX_EMAIL_BODY_SIZE_BYTES must be >= \
                        MAX_ATTACHMENT_SIZE_BYTES"
                .into());
        }
        Ok(())
    }
}

#[cfg(feature = "mailserver")]
impl SFeaturesConfig
{
    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        if self.enable_s3_storage
        {
            let s3 = self.s3_config.as_ref().ok_or(
                "ENABLE_S3_STORAGE is true but [s3_config] section missing"
            )?;

            if s3.bucket.is_empty()
            {
                return Err("s3_config.bucket cannot be empty".into());
            }
            if s3.region.is_empty()
            {
                return Err("s3_config.region cannot be empty".into());
            }
        }

        if self.encryption_enabled
        {
            let enc = self.encryption_config.as_ref().ok_or(
                "ENCRYPTION_ENABLED is true but [encryption_config] section \
                 missing"
            )?;

            if enc.algorithm.is_empty()
            {
                return Err(
                    "encryption_config.algorithm cannot be empty".into()
                );
            }

            if enc.key_rotation_days == 0 || enc.key_rotation_days > 365
            {
                return Err("encryption_config.key_rotation_days must be \
                            within [1, 365]"
                    .into());
            }
        }

        if self.self_hosted_mode && !self.encryption_enabled
        {
            eprintln!(
                "WARNING: SELF_HOSTED_MODE enabled without \
                 ENCRYPTION_ENABLED. Consider enabling encryption for \
                 self-hosted deployments."
            );
        }

        Ok(())
    }
}

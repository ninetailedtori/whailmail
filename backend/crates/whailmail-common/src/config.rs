// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # Configuration Structures — The Knobs & Dials (´｀)
//!
//! All the tunable stuff: database URLs, JWT secrets, log levels, mail server
//! defaults, connection pool sizes. This is where environment-specific behavior
//! lives.
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
    std::env
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SConfig
{
    pub db:          SDbConfig,
    pub jwt:         SJwtConfig,
    pub server:      SServerConfig,
    pub imap_sync:   SImapSyncConfig,
    pub smtp:        SSmtpConfig,
    pub mail_limits: SMailLimitsConfig,
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
pub struct SImapSyncConfig
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SFeaturesConfig
{
    pub self_hosted_mode:   bool,
    pub encryption_enabled: bool,
    pub enable_s3_storage:  bool
}

// Helpers

fn get_env<T>(
    name: &str,
    default: &str
) -> Result<T, Box<dyn std::error::Error>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + 'static
{
    env::var(name)
        .unwrap_or_else(|_| default.to_string())
        .parse::<T>()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

fn validate_range<T: PartialOrd + std::fmt::Display>(
    name: &str,
    value: T,
    min: T,
    max: T
) -> Result<(), Box<dyn std::error::Error>>
{
    if value < min || value > max
    {
        return Err(
            format!("{} must be between {} and {}", name, min, max).into()
        );
    }
    Ok(())
}

fn validate_nonzero<T: PartialEq + Default + std::fmt::Display>(
    name: &str,
    value: T
) -> Result<(), Box<dyn std::error::Error>>
{
    if value == T::default()
    {
        return Err(format!("{} must be > 0", name).into());
    }
    Ok(())
}

fn validate_gte<T: PartialOrd + std::fmt::Display>(
    name_a: &str,
    value_a: T,
    name_b: &str,
    value_b: T
) -> Result<(), Box<dyn std::error::Error>>
{
    if value_a < value_b
    {
        return Err(format!("{} must be >= {}", name_a, name_b).into());
    }
    Ok(())
}

fn validate_url_scheme(
    url: &str,
    schemes: &[&str]
) -> Result<(), Box<dyn std::error::Error>>
{
    if schemes.iter().any(|s| url.starts_with(s))
    {
        Ok(())
    }
    else
    {
        Err(format!("URL must start with one of: {:?}", schemes).into())
    }
}

// Config & Validation

impl SConfig
{
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        dotenv::dotenv().ok();

        let config = SConfig {
            db:          SDbConfig::from_env()?,
            jwt:         SJwtConfig::from_env()?,
            server:      SServerConfig::from_env()?,
            imap_sync:   SImapSyncConfig::from_env()?,
            smtp:        SSmtpConfig::from_env()?,
            mail_limits: SMailLimitsConfig::from_env()?,
            features:    SFeaturesConfig::from_env()?
        };

        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        self.db.validate()?;
        self.jwt.validate()?;
        self.server.validate()?;
        self.imap_sync.validate()?;
        self.smtp.validate()?;
        self.mail_limits.validate()?;
        Ok(())
    }
}

impl SDbConfig
{
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        let url = env::var("DB_URL").map_err(|_| {
            "DB_URL must be set (e.g., sqlite://whailmail.db or postgres://...)"
        })?;

        Ok(SDbConfig {
            url,
            max_connections: get_env("DB_MAX_CONNECTIONS", "5")?,
            min_idle: get_env("DB_MIN_IDLE", "1")?,
            connection_timeout_secs: get_env(
                "DB_CONNECTION_TIMEOUT_SECS",
                "10"
            )?
        })
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        validate_url_scheme(&self.url, &["sqlite://", "postgres://"])?;
        validate_range("DB_MAX_CONNECTIONS", self.max_connections, 1, 100)?;
        validate_range("DB_MIN_IDLE", self.min_idle, 0, self.max_connections)?;
        validate_range(
            "DB_CONNECTION_TIMEOUT_SECS",
            self.connection_timeout_secs,
            1,
            300
        )?;
        Ok(())
    }
}

impl SJwtConfig
{
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        let secret =
            env::var("JWT_SECRET").map_err(|_| "JWT_SECRET must be set")?;

        if secret.len() < 32
        {
            return Err("JWT_SECRET must be at least 32 characters".into());
        }

        let expiration_secs: u64 = get_env("JWT_EXPIRATION_SECS", "3600")?;
        let refresh_expiration_secs: u64 = get_env(
            "JWT_REFRESH_EXPIRATION_SECS",
            &(expiration_secs * 7).to_string()
        )?;

        Ok(SJwtConfig {
            secret,
            expiration_secs,
            refresh_expiration_secs
        })
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        validate_nonzero("JWT_EXPIRATION_SECS", self.expiration_secs)?;
        validate_gte(
            "JWT_REFRESH_EXPIRATION_SECS",
            self.refresh_expiration_secs,
            "JWT_EXPIRATION_SECS",
            self.expiration_secs
        )?;

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
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        let host = get_env::<String>("SERVER_HOST", "127.0.0.1")?;
        let port = get_env("SERVER_PORT", "8080")?;
        let log_level = get_env::<String>("LOG_LEVEL", "info")?;
        let cors_allowed_origins = Self::parse_cors_origins()?;

        Ok(SServerConfig {
            host,
            port,
            log_level,
            cors_allowed_origins
        })
    }

    fn parse_cors_origins() -> Result<Vec<String>, Box<dyn std::error::Error>>
    {
        let cors_str = env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| {
            "http://localhost:3000,http://localhost:5173".to_string()
        });

        Ok(cors_str.split(',').map(|s| s.trim().to_string()).collect())
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        validate_range("SERVER_PORT", self.port, 1, 65535)?;

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

impl SImapSyncConfig
{
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        Ok(SImapSyncConfig {
            poll_interval_secs:      get_env("IMAP_POLL_INTERVAL_SECS", "300")?,
            max_concurrent_syncs:    get_env("IMAP_MAX_CONCURRENT_SYNCS", "5")?,
            connection_timeout_secs: get_env(
                "IMAP_CONNECTION_TIMEOUT_SECS",
                "30"
            )?,
            idle_timeout_secs:       get_env("IMAP_IDLE_TIMEOUT_SECS", "600")?
        })
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        validate_range(
            "IMAP_POLL_INTERVAL_SECS",
            self.poll_interval_secs,
            1,
            u64::MAX
        )?;
        validate_range(
            "IMAP_MAX_CONCURRENT_SYNCS",
            self.max_concurrent_syncs,
            1,
            50
        )?;
        validate_range(
            "IMAP_CONNECTION_TIMEOUT_SECS",
            self.connection_timeout_secs,
            1,
            300
        )?;
        validate_gte(
            "IMAP_IDLE_TIMEOUT_SECS",
            self.idle_timeout_secs,
            "IMAP_CONNECTION_TIMEOUT_SECS",
            self.connection_timeout_secs
        )?;

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
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        Ok(SSmtpConfig {
            connection_timeout_secs: get_env(
                "SMTP_CONNECTION_TIMEOUT_SECS",
                "10"
            )?,
            send_timeout_secs:       get_env("SMTP_SEND_TIMEOUT_SECS", "30")?,
            max_retries:             get_env("SMTP_MAX_RETRIES", "3")?
        })
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        validate_range(
            "SMTP_CONNECTION_TIMEOUT_SECS",
            self.connection_timeout_secs,
            1,
            300
        )?;
        validate_range(
            "SMTP_SEND_TIMEOUT_SECS",
            self.send_timeout_secs,
            1,
            600
        )?;
        validate_range("SMTP_MAX_RETRIES", self.max_retries, 0, 10)?;

        Ok(())
    }
}

impl SMailLimitsConfig
{
    const MAX_ATTACHMENT_SIZE: u64 = 500_000_000;
    const MAX_BODY_SIZE: u64 = 1_000_000_000;
    const MAX_RECIPIENTS: u32 = 1000;
    const MIN_ATTACHMENT_SIZE: u64 = 1_000_000;
    const MIN_BODY_SIZE: u64 = 1_000_000;

    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        Ok(SMailLimitsConfig {
            max_attachment_size_bytes: get_env(
                "MAX_ATTACHMENT_SIZE_BYTES",
                "25000000"
            )?,
            max_email_body_size_bytes: get_env(
                "MAX_EMAIL_BODY_SIZE_BYTES",
                "100000000"
            )?,
            max_recipients_per_email:  get_env(
                "MAX_RECIPIENTS_PER_EMAIL",
                "100"
            )?
        })
    }

    fn validate(&self) -> Result<(), Box<dyn std::error::Error>>
    {
        validate_range(
            "MAX_ATTACHMENT_SIZE_BYTES",
            self.max_attachment_size_bytes,
            Self::MIN_ATTACHMENT_SIZE,
            Self::MAX_ATTACHMENT_SIZE
        )?;

        validate_range(
            "MAX_EMAIL_BODY_SIZE_BYTES",
            self.max_email_body_size_bytes,
            Self::MIN_BODY_SIZE,
            Self::MAX_BODY_SIZE
        )?;

        validate_range(
            "MAX_RECIPIENTS_PER_EMAIL",
            self.max_recipients_per_email,
            1,
            Self::MAX_RECIPIENTS
        )?;

        validate_gte(
            "MAX_EMAIL_BODY_SIZE_BYTES",
            self.max_email_body_size_bytes,
            "MAX_ATTACHMENT_SIZE_BYTES",
            self.max_attachment_size_bytes
        )?;

        Ok(())
    }
}

impl SFeaturesConfig
{
    fn from_env() -> Result<Self, Box<dyn std::error::Error>>
    {
        Ok(SFeaturesConfig {
            self_hosted_mode:   get_env("SELF_HOSTED_MODE", "false")?,
            encryption_enabled: get_env("ENCRYPTION_ENABLED", "false")?,
            enable_s3_storage:  get_env("ENABLE_S3_STORAGE", "false")?
        })
    }
}

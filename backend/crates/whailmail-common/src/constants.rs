// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Constants

pub mod app
{
    pub const NAME: &str = "whailmail";
    pub const FANCY_NAME: &str = "WhailMail";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const PROTOCOL_VERSION: &str = "1.0.0";
    pub const DB_SCHEMA_VERSION: u32 = 1;
}

pub mod api
{
    pub const API_VERSION: &str = "v1";
    pub const BASE_PATH: &str = "/api/v1";

    pub mod endpoints
    {
        pub const HEALTH: &str = "/health";
        pub const AUTH_LOGIN: &str = "/auth/login";
        pub const AUTH_LOGOUT: &str = "/auth/logout";
        pub const AUTH_REFRESH: &str = "/auth/refresh";
        pub const AUTH_REGISTER: &str = "/auth/register";

        pub const ACCOUNTS: &str = "/accounts";
        pub const ACCOUNTS_ID: &str = "/accounts/{id}";
        pub const ACCOUNTS_SYNC: &str = "/accounts/{id}/sync";
        pub const ACCOUNTS_VERIFY: &str = "/accounts/{id}/verify";

        pub const MAILBOXES: &str = "/mailboxes";
        pub const MAILBOXES_ID: &str = "/mailboxes/{id}";

        pub const EMAILS: &str = "/emails";
        pub const EMAILS_ID: &str = "/emails/{id}";
        pub const EMAILS_SEARCH: &str = "/emails/search";
        pub const EMAILS_SEND: &str = "/emails/send";
        pub const EMAILS_DRAFT: &str = "/emails/draft";

        pub const FILTERS: &str = "/filters";
        pub const FILTERS_ID: &str = "/filters/{id}";

        pub const SYNC: &str = "/sync";
        pub const SYNC_STATUS: &str = "/sync/status";
    }
}

pub mod ports
{
    pub const DEFAULT_API_PORT: u16 = 3000;
    pub const DEFAULT_SMTP_PORT: u16 = 25;
    pub const DEFAULT_SMTP_SUBMISSION_PORT: u16 = 587;
    pub const DEFAULT_SMTP_SUBMISSIONS_PORT: u16 = 465;
    pub const DEFAULT_IMAP_PORT: u16 = 143;
    pub const DEFAULT_IMAP_SSL_PORT: u16 = 993;
    pub const DEFAULT_WEB_PORT: u16 = 5173;
}

pub mod timeouts
{
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
    pub const DB_TIMEOUT_SECS: u64 = 15;
    pub const SMTP_TIMEOUT_SECS: u64 = 60;
    pub const IMAP_TIMEOUT_SECS: u64 = 45;
    pub const SYNC_TIMEOUT_SECS: u64 = 120;
    pub const JWT_EXPIRY_HOURS: i64 = 24;
    pub const REFRESH_TOKEN_EXPIRY_DAYS: i64 = 30;
}

pub mod limits
{
    pub const MAX_EMAIL_SIZE_MB: u64 = 25;
    pub const MAX_ATTACHMENT_SIZE_MB: u64 = 20;
    pub const MAX_ATTACHMENTS_PER_EMAIL: usize = 10;
    pub const MAX_RECIPIENTS: usize = 100;
    pub const MAX_EMAIL_BATCH_SIZE: usize = 50;
    pub const MAX_MAILBOXES_PER_ACCOUNT: usize = 50;
    pub const MAX_FILTERS_PER_ACCOUNT: usize = 100;
    pub const MAX_CONNECTIONS_PER_ACCOUNT: usize = 5;
    pub const PAGINATION_DEFAULT_LIMIT: u32 = 50;
    pub const PAGINATION_MAX_LIMIT: u32 = 500;
    pub const SEARCH_MAX_RESULTS: u32 = 1000;
    pub const PASSWORD_MIN_LENGTH: usize = 8;
    pub const USERNAME_MIN_LENGTH: usize = 3;
    pub const USERNAME_MAX_LENGTH: usize = 64;
}

pub mod rates
{
    pub const SMTP_RATE_LIMIT_PER_HOUR: u32 = 300;
    pub const LOGIN_ATTEMPT_LIMIT: u32 = 5;
    pub const LOGIN_ATTEMPT_WINDOW_MINUTES: u32 = 15;
    pub const API_RATE_LIMIT_PER_MINUTE: u32 = 60;
}

pub mod paths
{
    use std::path::PathBuf;

    enum DirType
    {
        Data,
        Config,
        Cache,
        Log
    }

    fn get_base_dir(dir_type: DirType) -> PathBuf
    {
        #[cfg(target_os = "windows")]
        {
            let env_var = match dir_type
            {
                | DirType::Cache | DirType::Log => "LOCALAPPDATA",
                | _ => "APPDATA"
            };
            let fallback = match dir_type
            {
                | DirType::Cache | DirType::Log => "APPDATA",
                | _ => "LOCALAPPDATA"
            };
            let base = std::env::var(env_var)
                .or_else(|_| std::env::var(fallback))
                .unwrap_or_else(|_| ".".to_string());
            PathBuf::from(base)
        }

        #[cfg(target_os = "macos")]
        {
            match dir_type
            {
                | DirType::Data =>
                {
                    dirs::data_local_dir().unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join(".local/share")
                    })
                },
                | DirType::Config =>
                {
                    dirs::config_dir().unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join("Library/Preferences")
                    })
                },
                | DirType::Cache =>
                {
                    dirs::cache_dir().unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join("Library/Caches")
                    })
                },
                | DirType::Log =>
                {
                    dirs::home_dir().unwrap_or_default().join("Library/Logs")
                },
            }
        }

        #[cfg(target_os = "ios")]
        {
            let app_id = crate::constants::app::NAME;
            match dir_type
            {
                | DirType::Data =>
                {
                    PathBuf::from(format!(
                        "/var/mobile/Containers/Data/Application/{}/Documents",
                        app_id
                    ))
                },
                | DirType::Config =>
                {
                    PathBuf::from(format!(
                        "/var/mobile/Containers/Data/Application/{}/Library",
                        app_id
                    ))
                },
                | DirType::Cache =>
                {
                    PathBuf::from(format!(
                        "/var/mobile/Containers/Data/Application/{}/Library/\
                         Caches",
                        app_id
                    ))
                },
                | DirType::Log =>
                {
                    PathBuf::from(format!(
                        "/var/mobile/Containers/Data/Application/{}/Documents/\
                         Logs",
                        app_id
                    ))
                },
            }
        }

        #[cfg(target_os = "android")]
        {
            match dir_type
            {
                | DirType::Data =>
                {
                    PathBuf::from(format!(
                        "/data/data/{}/files",
                        crate::constants::app::NAME
                    ))
                },
                | DirType::Config =>
                {
                    PathBuf::from(format!(
                        "/data/data/{}/shared_prefs",
                        crate::constants::app::NAME
                    ))
                },
                | DirType::Cache =>
                {
                    PathBuf::from(format!(
                        "/data/data/{}/cache",
                        crate::constants::app::NAME
                    ))
                },
                | DirType::Log =>
                {
                    PathBuf::from(format!(
                        "/data/data/{}/logs",
                        crate::constants::app::NAME
                    ))
                },
            }
        }

        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            let (env_var, fallback) = match dir_type
            {
                | DirType::Data => ("XDG_DATA_HOME", ".local/share"),
                | DirType::Config => ("XDG_CONFIG_HOME", ".config"),
                | DirType::Cache => ("XDG_CACHE_HOME", ".cache"),
                | DirType::Log => ("XDG_STATE_HOME", ".local/state")
            };

            let base = std::env::var(env_var).unwrap_or_else(|_| {
                dirs::home_dir()
                    .unwrap_or_default()
                    .join(fallback)
                    .to_string_lossy()
                    .to_string()
            });
            PathBuf::from(base)
        }
    }

    pub fn data_dir() -> PathBuf
    {
        get_base_dir(DirType::Data).join(crate::constants::app::NAME)
    }

    pub fn config_dir() -> PathBuf
    {
        get_base_dir(DirType::Config).join(crate::constants::app::NAME)
    }

    pub fn cache_dir() -> PathBuf
    {
        get_base_dir(DirType::Cache).join(crate::constants::app::NAME)
    }

    pub fn log_dir() -> PathBuf
    {
        let base = get_base_dir(DirType::Log);
        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            base.join(crate::constants::app::NAME).join("log")
        }
        #[cfg(not(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        )))]
        {
            base.join(crate::constants::app::NAME)
        }
    }

    pub fn certs_dir() -> PathBuf { config_dir().join("certs") }

    pub fn search_index_dir() -> PathBuf { cache_dir().join("search_index") }

    pub fn backups_dir() -> PathBuf { data_dir().join("backups") }

    pub fn db_path() -> PathBuf { data_dir().join("whailmail.db") }

    pub fn config_file() -> PathBuf { config_dir().join("config.toml") }
}

pub mod mail
{
    pub const RFC5322_MAX_LINE_LENGTH: usize = 998;
    pub const SMTP_BANNER_TIMEOUT_SECS: u64 = 5;
    pub const SMTP_COMMAND_TIMEOUT_SECS: u64 = 10;
    pub const IMAP_GREETING_TIMEOUT_SECS: u64 = 5;
    pub const IMAP_COMMAND_TIMEOUT_SECS: u64 = 30;
    pub const DKIM_SIGNATURE_ALGO: &str = "rsa-sha256";
    pub const DEFAULT_CHARSET: &str = "utf-8";
}

pub mod auth
{
    pub const JWT_ALGORITHM: &str = "HS256";
    pub const JWT_ISSUER: &str = "whailmail";
    pub const BCRYPT_COST: u32 = 12;
    pub const ARGON2_MEMORY: u32 = 65540;
    pub const ARGON2_ITERATIONS: u32 = 3;
    pub const ARGON2_PARALLELISM: u32 = 4;
}

pub mod database
{
    pub const DEFAULT_MIN_CONNECTIONS: u32 = 2;
    pub const DEFAULT_MAX_CONNECTIONS: u32 = 10;
    pub const DB_TIMEOUT_SECS: u64 = 15;
    pub const QUERY_TIMEOUT_SECS: u64 = 30;
}

pub mod sync
{
    pub const SYNC_INTERVAL_SECS: u64 = 300;
    pub const SYNC_BATCH_SIZE: u32 = 100;
    pub const FULL_SYNC_INTERVAL_DAYS: u32 = 7;
    pub const IDLE_TIMEOUT_SECS: u64 = 1200;
}

pub mod status
{
    pub const STATUS_ACTIVE: &str = "active";
    pub const STATUS_INACTIVE: &str = "inactive";
    pub const STATUS_SYNCING: &str = "syncing";
    pub const STATUS_ERROR: &str = "error";
    pub const STATUS_PENDING: &str = "pending";
}

pub mod headers
{
    pub const CONTENT_TYPE: &str = "content-type";
    pub const AUTHORIZATION: &str = "authorization";
    pub const X_REQUEST_ID: &str = "x-request-id";
    pub const X_CLIENT_VERSION: &str = "x-client-version";
    pub const X_API_VERSION: &str = "x-api-version";
}

pub mod defaults
{
    pub const LANG: &str = "en";
    pub const TIMEZONE: &str = "UTC";
    pub const THEME: &str = "catppuccin-macchiato";
}

pub mod cache
{
    pub const USER_CACHE_TTL_SECS: u64 = 3600;
    pub const ACCOUNT_CACHE_TTL_SECS: u64 = 1800;
    pub const MAILBOX_CACHE_TTL_SECS: u64 = 300;
    pub const EMAIL_CACHE_TTL_SECS: u64 = 600;
}

pub mod errors
{
    pub const ERR_INVALID_CREDENTIALS: &str = "Invalid credentials";
    pub const ERR_ACCOUNT_NOT_FOUND: &str = "Account not found";
    pub const ERR_USER_NOT_FOUND: &str = "User not found";
    pub const ERR_EMAIL_NOT_FOUND: &str = "Email not found";
    pub const ERR_MAILBOX_NOT_FOUND: &str = "Mailbox not found";
    pub const ERR_FILTER_NOT_FOUND: &str = "Filter not found";
    pub const ERR_UNAUTHORIZED: &str = "Unauthorized";
    pub const ERR_FORBIDDEN: &str = "Forbidden";
    pub const ERR_INVALID_INPUT: &str = "Invalid input";
    pub const ERR_DUPLICATE_ACCOUNT: &str = "Account already exists";
    pub const ERR_INVALID_EMAIL: &str = "Invalid email address";
    pub const ERR_INVALID_PASSWORD: &str = "Invalid password";
    pub const ERR_SYNC_FAILED: &str = "Sync failed";
    pub const ERR_SEND_FAILED: &str = "Failed to send email";
    pub const ERR_STORAGE_FULL: &str = "Storage full";
    pub const ERR_RATE_LIMITED: &str = "Rate limited";
    pub const ERR_SERVICE_UNAVAILABLE: &str = "Service unavailable";
    pub const ERR_INTERNAL_SERVER_ERROR: &str = "Internal server error";
}

pub mod regex
{
    pub const EMAIL_REGEX: &str = r#"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"#;
    pub const USERNAME_REGEX: &str = r"^[a-zA-Z0-9_-]{3,64}$";
    pub const DOMAIN_REGEX: &str = r"^([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$";
    pub const HOSTNAME_REGEX: &str = r"^([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$";
}

pub mod mimetypes
{
    pub const APPLICATION_JSON: &str = "application/json";
    pub const APPLICATION_OCTET_STREAM: &str = "application/octet-stream";
    pub const TEXT_PLAIN: &str = "text/plain";
    pub const TEXT_HTML: &str = "text/html";
    pub const MULTIPART_MIXED: &str = "multipart/mixed";
    pub const MULTIPART_ALTERNATIVE: &str = "multipart/alternative";
    pub const MULTIPART_RELATED: &str = "multipart/related";
}

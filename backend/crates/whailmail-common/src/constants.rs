// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # Application Constants — The Rules of the Road (´▽`)
//!
//! All the magic numbers that shouldn't change at runtime: max attachment
//! sizes, sync intervals, password policy, JWT expiration, protocol defaults.
//!
//! **For example:**
//! - Default IMAP fetch batch size
//! - Max email body length
//! - JWT token lifetime
//! - Rate limit defaults
//! - SMTP/IMAP default ports & timeouts
//!
//! Keeping these centralized means you only need to change one place to tweak
//! system-wide behavior.

pub mod app
{
    pub const NAME: &str = "whailmail";
    pub const FANCY_NAME: &str = "WhailMail";
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
}

pub mod crypto
{
    pub const JWT_ALGORITHM: &str = "HS256";
    pub const JWT_ISSUER: &str = "whailmail";
    pub const BCRYPT_COST: u32 = 12;
    pub const ARGON2_MEMORY: u32 = 65540;
    pub const ARGON2_ITERATIONS: u32 = 3;
    pub const ARGON2_PARALLELISM: u32 = 4;
    pub const DKIM_SIGNATURE_ALGO: &str = "rsa-sha256";
}

pub mod standards
{
    pub const RFC5322_MAX_LINE_LENGTH: usize = 998;
    pub const DEFAULT_CHARSET: &str = "utf-8";
    pub const PROTOCOL_VERSION: &str = "1.0.0";
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

pub mod headers
{
    pub const CONTENT_TYPE: &str = "content-type";
    pub const AUTHORIZATION: &str = "authorization";
    pub const X_REQUEST_ID: &str = "x-request-id";
    pub const X_CLIENT_VERSION: &str = "x-client-version";
    pub const X_API_VERSION: &str = "x-api-version";
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

pub mod status
{
    pub const ACTIVE: &str = "active";
    pub const INACTIVE: &str = "inactive";
    pub const SYNCING: &str = "syncing";
    pub const ERROR: &str = "error";
    pub const PENDING: &str = "pending";
}

pub mod errors
{
    pub const INVALID_CREDENTIALS: &str = "Invalid credentials";
    pub const ACCOUNT_NOT_FOUND: &str = "Account not found";
    pub const USER_NOT_FOUND: &str = "User not found";
    pub const EMAIL_NOT_FOUND: &str = "Email not found";
    pub const MAILBOX_NOT_FOUND: &str = "Mailbox not found";
    pub const FILTER_NOT_FOUND: &str = "Filter not found";
    pub const UNAUTHORIZED: &str = "Unauthorised";
    pub const FORBIDDEN: &str = "Forbidden";
    pub const INVALID_INPUT: &str = "Invalid input";
    pub const DUPLICATE_ACCOUNT: &str = "Account already exists";
    pub const INVALID_EMAIL: &str = "Invalid email address";
    pub const INVALID_PASSWORD: &str = "Invalid password";
    pub const SYNC_FAILED: &str = "Sync failed";
    pub const SEND_FAILED: &str = "Failed to send email";
    pub const STORAGE_FULL: &str = "Storage full";
    pub const RATE_LIMITED: &str = "Rate limited";
    pub const SERVICE_UNAVAILABLE: &str = "Service unavailable";
    pub const INTERNAL_SERVER_ERROR: &str = "Internal server error";
}

pub mod regex
{
    pub const EMAIL_REGEX: &str = r#"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$"#;
    pub const USERNAME_REGEX: &str = r"^[a-zA-Z0-9_-]{3,64}$";
    pub const DOMAIN_REGEX: &str = r"^([a-z0-9]+(-[a-z0-9]+)*\.)+[a-z]{2,}$";
    pub const HOSTNAME_REGEX: &str = r"^([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)*[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$";
}

pub mod paths
{
    use std::path::PathBuf;

    #[derive(Debug, Clone, Copy)]
    enum EDirType
    {
        Data,
        Config,
        Cache,
        Log
    }

    const APP_NAME: &str = crate::constants::app::NAME;

    macro_rules! xdg_dir {
        ($env:expr, $fallback:expr) => {{
            std::env::var($env)
                .ok()
                .map(PathBuf::from)
                .or_else(|| dirs::home_dir().map(|h| h.join($fallback)))
                .unwrap_or_default()
        }};
    }

    macro_rules! windows_dir {
        ($env_primary:expr, $env_fallback:expr) => {{
            std::env::var($env_primary)
                .or_else(|_| std::env::var($env_fallback))
                .ok()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."))
        }};
    }

    fn get_base_dir(dir_type: EDirType) -> PathBuf
    {
        #[cfg(target_os = "windows")]
        {
            match dir_type
            {
                | EDirType::Config => windows_dir!("APPDATA", "APPDATA"),
                | _ => windows_dir!("LOCALAPPDATA", "APPDATA")
            }
        }

        #[cfg(target_os = "macos")]
        {
            match dir_type
            {
                | EDirType::Data =>
                {
                    dirs::data_local_dir().unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join("Library/Application Support")
                    })
                },
                | EDirType::Config =>
                {
                    dirs::config_dir().unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join("Library/Preferences")
                    })
                },
                | EDirType::Cache =>
                {
                    dirs::cache_dir().unwrap_or_else(|| {
                        dirs::home_dir()
                            .unwrap_or_default()
                            .join("Library/Caches")
                    })
                },
                | EDirType::Log =>
                {
                    dirs::home_dir().unwrap_or_default().join("Library/Logs")
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
            match dir_type
            {
                | EDirType::Data => xdg_dir!("XDG_DATA_HOME", ".local/share"),
                | EDirType::Config => xdg_dir!("XDG_CONFIG_HOME", ".config"),
                | EDirType::Cache => xdg_dir!("XDG_CACHE_HOME", ".cache"),
                | EDirType::Log => xdg_dir!("XDG_STATE_HOME", ".local/state")
            }
        }
    }

    // Single-path accessors for data/cache/log
    pub fn data_dir() -> PathBuf { get_base_dir(EDirType::Data).join(APP_NAME) }

    pub fn cache_dir() -> PathBuf
    {
        get_base_dir(EDirType::Cache).join(APP_NAME)
    }

    pub fn log_dir() -> PathBuf
    {
        let base = get_base_dir(EDirType::Log);
        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            base.join(APP_NAME).join("log")
        }
        #[cfg(not(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        )))]
        {
            base.join(APP_NAME)
        }
    }

    pub fn config_search_paths() -> Vec<PathBuf>
    {
        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        {
            vec![
                xdg_dir!("XDG_CONFIG_HOME", ".config").join(APP_NAME),
                PathBuf::from(format!("/etc/{}", APP_NAME)),
                PathBuf::from(format!("/usr/local/share/{}", APP_NAME)),
                PathBuf::from(format!("/usr/share/{}", APP_NAME)),
            ]
        }

        #[cfg(target_os = "macos")]
        {
            vec![
                dirs::config_dir().unwrap_or_default().join(APP_NAME),
                PathBuf::from(format!("/etc/{}", APP_NAME)),
            ]
        }

        #[cfg(target_os = "windows")]
        {
            vec![
                windows_dir!("APPDATA", "APPDATA").join(APP_NAME),
                PathBuf::from(format!("C:\\ProgramData\\{}", APP_NAME)),
            ]
        }
    }

    pub fn config_dir() -> PathBuf
    {
        get_base_dir(EDirType::Config).join(APP_NAME)
    }

    pub fn certs_dir() -> PathBuf { config_dir().join("certs") }

    pub fn search_index_dir() -> PathBuf { cache_dir().join("search_index") }

    pub fn backups_dir() -> PathBuf { data_dir().join("backups") }

    pub fn db_path() -> PathBuf { data_dir().join("whailmail.db") }

    pub fn config_file() -> PathBuf
    {
        config_search_paths()
            .into_iter()
            .find(|p| p.join("config.toml").exists())
            .map(|p| p.join("config.toml"))
            .unwrap_or_else(|| {
                get_base_dir(EDirType::Config)
                    .join(APP_NAME)
                    .join("config.toml")
            })
    }
}

// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! (´・ω・`) Error handling for whailmail
//!
//! This module defines `EAppError`, the unified error type that flows through
//! the entire application — from low-level protocol crates (IMAP, SMTP,
//! indexing) all the way up to HTTP responses sent to the Electron frontend.
//!
//! ## Philosophy
//!
//! Rather than letting errors scatter across the codebase with inconsistent
//! handling, we wrap everything into semantic variants that know:
//! - **What HTTP status code to return** (401, 503, etc.)
//! - **Whether it's worth retrying** (transient vs. permanent)
//! - **How to log it appropriately** (debug, warn, error — with context)
//!
//! ## Error Layering
//!
//! Errors bubble up from low-level crates and get wrapped with business
//! context:
//!
//! ```ignore
//! IMAP fails to connect
//!   - ImapConnectionError(anyhow::Error)
//!   - .context("Failed to fetch inbox")?
//!   - Context { message, source: ImapConnectionError }
//!   - Axum handler logs + serializes to JSON
//! ```
//!
//! The `Context` variant lets you attach human-readable context at any layer
//! without losing the original error type. Your `status_code()` walks the chain
//! to find the root cause.
//!
//! ## Usage
//!
//! Wrap external errors in protocol-specific variants, add context in handlers:
//!
//! ```ignore
//! // Low-level crate
//! pub async fn fetch(&self) -> RAppResult<Vec<Message>> {
//!     self.connection.fetch().await
//!         .map\_err(|e| EAppError::ImapCommandError(anyhow::Error::from(e)))
//! }
//!
//! // Handler
//! pub async fn get\_inbox(State(imap): State<ImapClient>)
//! -> Result<Json<Vec<Message>>, EAppError> {
//!     imap.fetch()
//!         .await
//!         .context("Failed to fetch user inbox")?
//! }
//! ```
//!
//! The frontend gets a clean error chain instead of a Rust backtrace. Everyone
//! wins.

use {
    serde_json::json,
    std::error::Error,
    thiserror::Error,
    tracing::{debug, error, warn}
};

#[derive(Error, Debug)]
pub enum EAppError
{
    // 4xx - Client Errors

    // 400 - Bad Request
    #[error("Validation error")]
    ValidationError(String),

    #[error("Invalid request")]
    InvalidRequest(String),

    #[error("Mail parsing failed")]
    MailParsingError(#[source] anyhow::Error),

    // 401 - Unauthorized
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Invalid token format")]
    InvalidToken,

    #[error("JWT token expired")]
    TokenExpired,

    #[error("IMAP authentication failed")]
    ImapAuthError(#[source] anyhow::Error),

    #[error("SMTP authentication failed")]
    SmtpAuthError(#[source] anyhow::Error),

    // 403 - Forbidden
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Insufficient permissions")]
    Forbidden,

    #[error("TLS/SSL error")]
    TlsError(#[source] anyhow::Error),

    #[error("Account locked")]
    AccountLocked,

    // 404 - Not Found
    #[error("Resource not found")]
    NotFound(String),

    #[error("Mailbox not found")]
    MailboxNotFound(String),

    // 409 - Conflict
    #[error("Resource already exists")]
    DuplicateResource(String),

    #[error("Conflict")]
    Conflict(String),

    // 413 - Payload Too Large
    #[error("File too large")]
    FileTooLarge(String),

    // 415 - Unsupported Media Type
    #[error("Unsupported media type")]
    UnsupportedMediaType,

    // 422 - Unprocessable Entity
    #[error("Invalid recipient")]
    InvalidRecipient(String),

    // 429 - Too Many Requests
    #[error("Rate limit exceeded")]
    RateLimited,

    #[error("Quota exceeded")]
    QuotaExceeded(String),

    // 5xx - Server Errors

    // 500 - Internal Server Error
    #[error("Mail encoding failed")]
    MailEncodingError(#[source] anyhow::Error),

    #[error("Filter processing failed")]
    FilterError(#[source] anyhow::Error),

    #[error("Database error")]
    DatabaseError(#[source] anyhow::Error),

    #[error("Configuration error")]
    ConfigError(#[source] anyhow::Error),

    #[error("File system error")]
    FileSystemError(#[source] anyhow::Error),

    #[error("Internal server error")]
    InternalError(#[source] anyhow::Error),

    // 501 - Not Implemented
    #[error("Feature not implemented")]
    NotImplemented,

    // 503 - Service Unavailable
    #[error("IMAP connection failed")]
    ImapConnectionError(#[source] anyhow::Error),

    #[error("IMAP command failed")]
    ImapCommandError(#[source] anyhow::Error),

    #[error("SMTP connection failed")]
    SmtpConnectionError(#[source] anyhow::Error),

    #[error("SMTP send failed")]
    SmtpSendError(#[source] anyhow::Error),

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("Indexing failed")]
    IndexError(#[source] anyhow::Error),

    #[error("Search failed")]
    SearchError(#[source] anyhow::Error),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Service not initialized")]
    ServiceNotInitialized,

    // 504 - Gateway Timeout
    #[error("Request timeout")]
    Timeout,

    // Custom Context
    #[error("{message}")]
    Context
    {
        message: String,
        #[source]
        source:  Option<Box<EAppError>>
    }
}

/// Helper trait for converting any error to EAppError with context
trait TErrorContext<T>
{
    fn context<S: ToString>(self, msg: S) -> anyhow::Result<T>;
}

impl<T> TErrorContext<T> for RAppResult<T>
{
    fn context<S: ToString>(self, msg: S) -> anyhow::Result<T>
    {
        self.map_err(|err| anyhow::anyhow!("{}: {}", msg.to_string(), err))
    }
}

impl EAppError
{
    pub fn status_code(&self) -> u16
    {
        match self
        {
            // 4xx - Client Errors

            // 400 - Bad Request
            | EAppError::ValidationError(_)
            | EAppError::InvalidRequest(_)
            | EAppError::MailParsingError(_) => 400,

            // 401 - Unauthorized
            | EAppError::InvalidCredentials
            | EAppError::InvalidToken
            | EAppError::TokenExpired
            | EAppError::ImapAuthError(_)
            | EAppError::SmtpAuthError(_) => 401,

            // 403 - Forbidden
            | EAppError::Unauthorized
            | EAppError::Forbidden
            | EAppError::TlsError(_) => 403,

            // 404 - Not Found
            | EAppError::NotFound(_) | EAppError::MailboxNotFound(_) => 404,

            // 409 - Conflict
            | EAppError::DuplicateResource(_) | EAppError::Conflict(_) => 409,

            // 413 - Payload Too Large
            | EAppError::FileTooLarge(_) => 413,

            // 415 - Unsupported Media Type
            | EAppError::UnsupportedMediaType => 415,

            // 422 - Unprocessable Entity
            | EAppError::InvalidRecipient(_) => 422,

            // 423 - Locked
            | EAppError::AccountLocked => 423,

            // 429 - Too Many Requests
            | EAppError::RateLimited | EAppError::QuotaExceeded(_) => 429,

            // 5xx - Server Errors

            // 500 - Internal Server Error
            | EAppError::MailEncodingError(_)
            | EAppError::FilterError(_)
            | EAppError::DatabaseError(_)
            | EAppError::ConfigError(_)
            | EAppError::FileSystemError(_)
            | EAppError::InternalError(_) => 500,

            // 501 - Not Implemented
            | EAppError::NotImplemented => 501,

            // 503 - Service Unavailable
            | EAppError::ImapConnectionError(_)
            | EAppError::ImapCommandError(_)
            | EAppError::SmtpConnectionError(_)
            | EAppError::SmtpSendError(_)
            | EAppError::ConnectionRefused
            | EAppError::IndexError(_)
            | EAppError::SearchError(_)
            | EAppError::ServiceUnavailable
            | EAppError::ServiceNotInitialized => 503,

            // 504 - Gateway Timeout
            | EAppError::Timeout => 504,

            // Custom Context
            | EAppError::Context {
                source, ..
            } => source.as_ref().map(|s| (**s).status_code()).unwrap_or(500)
        }
    }

    pub fn is_retryable(&self) -> bool
    {
        matches!(
            self,
            // Network/transient
            EAppError::Timeout
            | EAppError::ConnectionRefused
            | EAppError::ServiceUnavailable
            | EAppError::ServiceNotInitialized
            | EAppError::ImapConnectionError(_)
            | EAppError::ImapCommandError(_)
            | EAppError::SmtpConnectionError(_)
            | EAppError::SmtpSendError(_)
            | EAppError::IndexError(_)
            | EAppError::SearchError(_)
            // Rate limiting (with backoff)
            | EAppError::RateLimited
            // Possibly transient database issues
            | EAppError::DatabaseError(_)
        )
    }

    /// Log the error with full chain
    pub fn log(&self)
    {
        match self
        {
            // Expected client errors - debug or no log
            | EAppError::ValidationError(_)
            | EAppError::InvalidRequest(_)
            | EAppError::NotFound(_)
            | EAppError::MailboxNotFound(_)
            | EAppError::NotImplemented =>
            {
                debug!(error = %self, "Client request rejected");
            },

            // Auth/access failures - warn (security/compliance relevant)
            | EAppError::InvalidCredentials
            | EAppError::InvalidToken
            | EAppError::TokenExpired
            | EAppError::Unauthorized
            | EAppError::Forbidden
            | EAppError::AccountLocked =>
            {
                warn!(error = %self, "Authentication/authorization failed");
            },

            // Transient system errors - error + retryable
            | EAppError::Timeout
            | EAppError::ConnectionRefused
            | EAppError::ServiceNotInitialized =>
            {
                error!(error = %self, retryable = true, "Transient error");
            },

            // Actual failures - error with context
            | _ =>
            {
                error!(
                    error = %self,
                    error_chain = ?self.source(),
                    retryable = self.is_retryable(),
                    "Server error"
                );
            }
        }
    }

    /// Get error chain (including source errors)
    fn unwind(&self) -> Vec<String>
    {
        let mut chain = vec![self.to_string()];
        let mut source = self.source();

        while let Some(err) = source
        {
            chain.push(err.to_string());
            source = err.source();
        }

        chain
    }

    /// JSON representation for API responses
    pub fn to_json(&self) -> serde_json::Value
    {
        json!({
            "error": self.to_string(),
            "chain": self.unwind(),
            "code": self.status_code(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "retryable": self.is_retryable(),
        })
    }
}

impl From<std::io::Error> for EAppError
{
    fn from(err: std::io::Error) -> Self
    {
        match err.kind()
        {
            | std::io::ErrorKind::TimedOut => EAppError::Timeout,
            | std::io::ErrorKind::ConnectionRefused =>
            {
                EAppError::ConnectionRefused
            },
            | _ => EAppError::FileSystemError(anyhow::Error::from(err))
        }
    }
}

impl From<anyhow::Error> for EAppError
{
    fn from(err: anyhow::Error) -> Self { EAppError::InternalError(err) }
}

impl From<serde_json::error::Error> for EAppError
{
    fn from(err: serde_json::error::Error) -> Self
    {
        EAppError::InvalidRequest(err.to_string())
    }
}

pub type RAppResult<T> = Result<T, EAppError>;

impl EAppError
{
    pub fn into_anyhow(self) -> anyhow::Error { anyhow::anyhow!("{}", self) }
}

// error.rs
use {
    anyhow::Context,
    serde_json::json,
    std::fmt,
    thiserror::Error,
    tracing::{error, warn}
};

#[derive(Error, Debug)]
pub enum EAppError
{
    // Auth & JWT
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("JWT token expired")]
    TokenExpired,

    #[error("Invalid token format")]
    InvalidToken,

    // Validation
    #[error("Validation error")]
    ValidationError(String),

    // Not found
    #[error("Record not found")]
    NotFound(String),

    // Conflicts
    #[error("Resource already exists")]
    DuplicateResource(String),

    #[error("Conflict")]
    Conflict(String),

    // Protocol - IMAP
    #[error("IMAP connection failed")]
    ImapConnectionError(#[source] anyhow::Error),

    #[error("IMAP authentication failed")]
    ImapAuthError(#[source] anyhow::Error),

    #[error("IMAP command failed")]
    ImapCommandError(#[source] anyhow::Error),

    // Protocol - SMTP
    #[error("SMTP connection failed")]
    SmtpConnectionError(#[source] anyhow::Error),

    #[error("SMTP authentication failed")]
    SmtpAuthError(#[source] anyhow::Error),

    #[error("SMTP send failed")]
    SmtpSendError(#[source] anyhow::Error),

    #[error("Invalid recipient")]
    InvalidRecipient(String),

    // Mail processing
    #[error("Mail parsing failed")]
    MailParsingError(#[source] anyhow::Error),

    #[error("Mail encoding failed")]
    MailEncodingError(#[source] anyhow::Error),

    #[error("Filter processing failed")]
    FilterError(#[source] anyhow::Error),

    // Indexing
    #[error("Indexing failed")]
    IndexError(#[source] anyhow::Error),

    #[error("Search failed")]
    SearchError(#[source] anyhow::Error),

    // Network
    #[error("Request timeout")]
    Timeout,

    #[error("Connection refused")]
    ConnectionRefused,

    #[error("TLS/SSL error")]
    TlsError(#[source] anyhow::Error),

    // Resource limits
    #[error("File too large")]
    FileTooLarge(String),

    #[error("Quota exceeded")]
    QuotaExceeded(String),

    #[error("Rate limit exceeded")]
    RateLimited,

    // API & requests
    #[error("Invalid request")]
    InvalidRequest(String),

    #[error("Unsupported media type")]
    UnsupportedMediaType,

    // Server
    #[error("Database error")]
    DatabaseError(#[source] anyhow::Error),

    #[error("Configuration error")]
    ConfigError(#[source] anyhow::Error),

    #[error("File system error")]
    FileSystemError(#[source] anyhow::Error),

    #[error("Internal server error")]
    InternalError(#[source] anyhow::Error),

    #[error("Service unavailable")]
    ServiceUnavailable
}

impl EAppError
{
    pub fn status_code(&self) -> u16
    {
        match self
        {
            // 4xx
            | EAppError::InvalidCredentials
            | EAppError::InvalidToken
            | EAppError::TokenExpired => 401,
            | EAppError::Unauthorized => 403,
            | EAppError::NotFound(_) => 404,
            | EAppError::DuplicateResource(_) | EAppError::Conflict(_) => 409,
            | EAppError::ValidationError(_) | EAppError::InvalidRequest(_) =>
            {
                400
            },
            | EAppError::UnsupportedMediaType => 415,
            | EAppError::FileTooLarge(_) => 413,
            | EAppError::InvalidRecipient(_) => 422,
            | EAppError::RateLimited => 429,
            // 5xx
            | EAppError::Timeout => 504,
            | EAppError::ServiceUnavailable => 503,
            | _ => 500
        }
    }

    pub fn is_retryable(&self) -> bool
    {
        matches!(
            self,
            EAppError::Timeout
                | EAppError::ConnectionRefused
                | EAppError::ServiceUnavailable
                | EAppError::ImapConnectionError(_)
                | EAppError::SmtpConnectionError(_)
        )
    }

    /// Log the error with full chain
    pub fn log(&self)
    {
        match self
        {
            // Auth errors → warning level
            | EAppError::InvalidCredentials
            | EAppError::InvalidToken
            | EAppError::TokenExpired
            | EAppError::Unauthorized =>
            {
                warn!(error = %self, error_chain = ?self.source(), "Authentication failed");
            },
            // Client errors → info level
            | EAppError::ValidationError(_)
            | EAppError::InvalidRequest(_)
            | EAppError::NotFound(_) =>
            {
                warn!(error = %self, "Client error");
            },
            // Server/retryable errors → error level
            | EAppError::Timeout | EAppError::ConnectionRefused =>
            {
                error!(error = %self, retryable = true, "Transient error");
            },
            // Everything else → error level
            | _ =>
            {
                error!(
                    error = %self,
                    error_chain = ?self.source(),
                    retryable = self.is_retryable(),
                    "Error occurred"
                );
            }
        }
    }

    /// JSON representation for API responses
    pub fn to_json(&self) -> serde_json::Value
    {
        json!({
            "error": self.to_string(),
            "code": self.status_code(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    }
}

// Helper trait for converting any error to EAppError with context
pub trait ErrorContext<T>
{
    fn app_context(self, msg: &str) -> anyhow::Result<T>;
}

impl<T, E> ErrorContext<T> for Result<T, E>
where E: std::error::Error + Send + Sync + 'static
{
    fn app_context(self, msg: &str) -> anyhow::Result<T> { self.context(msg) }
}

// From implementations for external crates
impl From<tokio::io::Error> for EAppError
{
    fn from(err: tokio::io::Error) -> Self
    {
        match err.kind()
        {
            | std::io::ErrorKind::TimedOut => EAppError::Timeout,
            | std::io::ErrorKind::ConnectionRefused =>
            {
                EAppError::ConnectionRefused
            },
            | _ => EAppError::InternalError(anyhow::Error::from(err))
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

// Standard library implementations
impl From<std::io::Error> for EAppError
{
    fn from(err: std::io::Error) -> Self
    {
        EAppError::FileSystemError(anyhow::Error::from(err))
    }
}

pub type RAppResult<T> = Result<T, EAppError>;

// Bonus: Convert Result<T, EAppError> → Result<T, anyhow::Error>
impl From<EAppError> for anyhow::Error
{
    fn from(err: EAppError) -> Self { anyhow::anyhow!("{}", err) }
}

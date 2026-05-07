// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Exceptions

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EAppError
{
    // Auth
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("JWT token expired")]
    TokenExpired,

    // Validation
    #[error("Validation error: {0}")]
    ValidationError(String),

    // Not found
    #[error("Record not found: {0}")]
    NotFound(String),

    // Protocol
    #[error("IMAP connection failed: {0}")]
    ImapConnectionError(String),

    #[error("SMTP connection failed: {0}")]
    SmtpConnectionError(String),

    #[error("Mail parsing error: {0}")]
    MailParsingError(String),

    // Server
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Internal server error: {0}")]
    InternalError(String)
}

impl EAppError
{
    pub fn status_code(&self) -> u16
    {
        match self
        {
            | EAppError::NotFound(_) => 404,
            | EAppError::InvalidCredentials => 401,
            | EAppError::Unauthorized => 403,
            | EAppError::ValidationError(_) => 400,
            | EAppError::TokenExpired => 401,
            | _ => 500
        }
    }
}

pub type RAppResult<T> = Result<T, EAppError>;

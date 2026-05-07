//! WhailMail's common types and utils

pub mod config;
pub mod constants;
pub mod dto;
pub mod error;
pub mod models;

pub use config::Config;
pub use error::{AppError, AppResult};
pub use models::*;

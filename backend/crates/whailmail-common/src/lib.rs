/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! WhailMail's common types and utils

pub mod config;
pub mod constants;
pub mod dto;
pub mod error;
pub mod types;

pub use config::Config;
pub use error::{AppError, AppResult};
pub use models::*;

// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>

// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # WhailMail Common — Shared Domain Layer (´・ω・`)
//!
//! Your single import point for everything WhailMail needs across crates.
//!
//! This is the heart of our type system: domain models, errors, DTOs, config,
//! and constants all live here. No business logic, no I/O — just the vocabulary
//! we use to talk about users, accounts, emails, and filters.
//!
//! **Use this to:** Import core types in `whailmail-db`, `whailmail-api`, etc.
//!
//! ```ignore
//! use whailmail\_common::{SUser, SEmail, EAppError, RAppResult};
//! ```

pub mod config;
pub mod constants;
pub mod dto;
pub mod error;
pub mod types;

pub use {
    config::SConfig,
    error::{EAppError, RAppResult},
    types::*
};

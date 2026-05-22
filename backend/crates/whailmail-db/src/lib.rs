// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! # WhailMail Database Layer ૮꒰ ˶• ༝ •˶꒱ა ♡
//!
//! SQL queries with connection pooling.
//! Wraps SQLite (dev) and PostgreSQL (prod) behind a unified interface, this is
//! everything to us! All our data lies in DBs!
//!
//! **Modules:**
//! - `pool` — Connection pooling + setup
//! - `models` — Domain structs
//! - `queries` — Raw SQL query builders & executors
//! - `repositories` — High-level data access patterns
//! - `migrations` — Schema setup

pub use whailmail_common::{
    config::{SConfig, SDbConfig},
    error::EAppError,
    types::*
};

pub mod migrations;
pub mod pool;
pub mod queries;
pub mod repositories;

pub use pool::SPool;
pub type RDbResult<T> = Result<T, EAppError>;

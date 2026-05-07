/*
 * SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
 * SPDX-FileContributor: WhailMail contributors
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

//! Config

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SConfig {
    pub database: SDbConfig,
    pub jwt: SJwtConfig,
    pub server: SServerConfig,
    pub imap_sync: SImapSyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SDbConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_idle: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SJwtConfig {
    pub secret: String,
    pub expiration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SServerConfig {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SImapSyncConfig {
    pub poll_interval_secs: u64,
    pub max_concurrent_syncs: u32,
}

impl SConfig {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        // Load from .env or environment variables
        // Pseudocode:
        // let db_url = env::var("DATABASE_URL")?;
        // let jwt_secret = env::var("JWT_SECRET")?;
        // Ok(Config { ... })
        todo!()
    }
}

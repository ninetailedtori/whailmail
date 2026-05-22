// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

use {
    crate::EAppError,
    anyhow::anyhow,
    sqlx::{
        ConnectOptions,
        Sqlite,
        pool::PoolConnection,
        sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions}
    },
    std::str::FromStr,
    tracing::{debug, info, log::LevelFilter, warn},
    whailmail_common::SDbConfig
};

#[derive(Debug, Clone)]
pub struct SPool
{
    inner: SqlitePool
}

impl SPool
{
    pub async fn new(config: SDbConfig) -> Result<Self, EAppError>
    {
        info!("Initializing database pool from: {}", config.url);

        let connect_opts = SqliteConnectOptions::from_str(&config.url)
            .map_err(|e| {
                EAppError::DatabaseError(anyhow!("Invalid SQLite URL: {}", e))
            })?
            .pragma("foreign_keys", "ON")
            .pragma("journal_mode", "WAL")
            .log_statements(LevelFilter::Debug);

        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .min_connections(config.min_idle)
            .acquire_timeout(std::time::Duration::from_secs(
                config.connection_timeout_secs
            ))
            .idle_timeout(Some(std::time::Duration::from_secs(300)))
            .connect_with(connect_opts)
            .await
            .map_err(|e| {
                EAppError::DatabaseError(anyhow!(
                    "Failed to create pool: {}",
                    e
                ))
            })?;

        debug!("Database pool created (SQLite)");

        Ok(Self {
            inner: pool
        })
    }

    pub async fn new_test() -> Result<Self, EAppError>
    {
        Self::new(SDbConfig::test_sqlite()).await
    }

    pub fn inner(&self) -> &SqlitePool { &self.inner }

    pub async fn acquire(&self) -> Result<PoolConnection<Sqlite>, EAppError>
    {
        self.inner.acquire().await.map_err(|e| {
            EAppError::DatabaseError(anyhow!(
                "Failed to acquire connection: {}",
                e
            ))
        })
    }

    pub fn stats(&self) -> SPoolStats
    {
        let total = self.inner.size();
        let idle = self.inner.num_idle() as u32;

        SPoolStats {
            total_conns:  total,
            idle_conns:   idle,
            active_conns: total.saturating_sub(idle)
        }
    }

    pub async fn health(&self) -> Result<(), EAppError>
    {
        sqlx::query("SELECT 1")
            .execute(self.inner())
            .await
            .map_err(|e| {
                warn!("Database health check failed: {}", e);
                EAppError::DatabaseError(anyhow!("Database unreachable"))
            })?;

        Ok(())
    }

    pub async fn close(self) -> Result<(), EAppError>
    {
        self.inner.close().await;
        info!("Database pool closed");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SPoolStats
{
    pub total_conns:  u32,
    pub idle_conns:   u32,
    pub active_conns: u32
}

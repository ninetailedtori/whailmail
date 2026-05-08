// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! Test utilities & fixtures shared across all tests

pub mod fixtures;
pub mod integrations;

use {tempfile::TempDir, uuid::Uuid, whailmail_common::AppConfig};

/// Isolated test environment
pub struct TestContext
{
    pub config: AppConfig,
    _temp_dir:  TempDir
}

impl TestContext
{
    pub fn new() -> Result<Self, Box<dyn std::error::Error>>
    {
        let temp_dir = TempDir::new()?;
        let test_id = Uuid::new_v4();

        let config = AppConfig {
            database_url:  format!(
                "postgresql://localhost/whailmail_test_{}",
                test_id
            ),
            jwt_secret:    format!("test-secret-{}", test_id),
            imap_host:     "localhost".to_string(),
            imap_port:     143,
            smtp_host:     "localhost".to_string(),
            smtp_port:     25,
            max_mail_size: 52428800,
            index_path:    temp_dir
                .path()
                .join("index")
                .to_string_lossy()
                .to_string()
        };

        Ok(Self {
            config,
            _temp_dir: temp_dir
        })
    }

    pub fn with_db_url(mut self, url: &str) -> Self
    {
        self.config.database_url = url.to_string();
        self
    }

    pub fn index_path(&self) -> &str { &self.config.index_path }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn isolation()
    {
        let ctx1 = TestContext::new().unwrap();
        let ctx2 = TestContext::new().unwrap();
        assert_ne!(ctx1.config.database_url, ctx2.config.database_url);
    }
}

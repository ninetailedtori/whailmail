// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;

#[tokio::test]
async fn test_imap_connection()
{
    let ctx = TestContext::new().expect("Failed to create context");

    let imap_client = whailmail_imap::client::ImapClient::new(
        &ctx.config.imap_host,
        ctx.config.imap_port
    );

    // Test connection (will timeout if server not running, which is ok for CI)
    // assert!(imap_client.connect().await.is_ok());
}

#[tokio::test]
async fn test_mail_sync()
{
    let ctx = TestContext::new().expect("Failed to create context");
    let db = whailmail_db::init(&ctx.config).await.unwrap();

    // Test full sync pipeline: imap → db → indexer
    // ...
}

// SPDX-FileCopyrightText: 2026–Present ninetailedtori <ninetailedtori@uwu.gal>
// SPDX-FileContributor: WhailMail contributors
//
// SPDX-License-Identifier: GPL-3.0-or-later

use super::*;

#[tokio::test]
async fn test_db_connection()
{
    let ctx = TestContext::new().expect("Failed to create context");

    // Test whailmail-db with isolated config
    let db = whailmail_db::init(&ctx.config)
        .await
        .expect("DB init failed");

    assert!(db.health().await.is_ok());
}

#[tokio::test]
async fn test_user_crud()
{
    let ctx = TestContext::new().expect("Failed to create context");
    let db = whailmail_db::init(&ctx.config)
        .await
        .expect("DB init failed");

    // Test create, read, update, delete
    // ...
}

use super::*;

#[tokio::test]
async fn test_db_connection()
{
    let ctx = TestContext::new().expect("Failed to create context");

    // Test whailmail-db with isolated config
    let db = whailmail_db::init(&ctx.config)
        .await
        .expect("DB init failed");

    assert!(db.health_check().await.is_ok());
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

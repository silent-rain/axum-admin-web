//! Mock 测试

mod common;
use common::{Role, User};

use database::mock::Mock;
use sea_orm::{ConnectionTrait, DbBackend, DbErr, EntityTrait, Statement};
use std::env;
use tracing::info;

async fn mock_pool() -> Result<(), DbErr> {
    let pool = Mock::builder().await?.build();

    // 创建表并返回pool
    let sql = r#"CREATE TABLE `user` 
    (
        `id` INT NULL,
        `name` TEXT NOT NULL,
        `status` BOOLEAN NOT NULL,
        PRIMARY KEY (`id`)
    );"#;
    let result = pool.db().execute_unprepared(sql).await?;
    info!("result1: {:#?}", result);

    // 插入数据
    let sql = r#"INSERT INTO user (id,name, status)
     VALUES (1,'zhangsan',true);"#;
    pool.db().execute_unprepared(sql).await?;

    // 查看数据
    let stmt = Statement::from_sql_and_values(DbBackend::MySql, r#"SELECT * FROM  user"#, []);
    let result = pool.db().query_one(stmt).await?;
    info!("result2: {:#?}", result);
    assert!(result.is_some());

    // 获取数据
    let user = User::find().one(pool.db()).await?;
    info!("user: {:#?}", user);
    assert!(user.is_some());
    Ok(())
}

async fn mock_entity() -> Result<(), DbErr> {
    // 创建表并返回pool
    let pool = Mock::builder()
        .await?
        .migration_entity(User)
        .await?
        .migration_entity(Role)
        .await?
        .build();

    // 插入数据
    let sql = r#"INSERT INTO user (id,name, status)
     VALUES (1,'zhangsan',1);"#;
    pool.db().execute_unprepared(sql).await?;

    // 查看数据
    let stmt = Statement::from_sql_and_values(DbBackend::Sqlite, r#"SELECT * FROM  user"#, []);
    let result = pool.db().query_one(stmt).await?;
    info!("result: {:#?}", result);
    assert!(result.is_some());

    // 获取数据
    let user = User::find().one(pool.db()).await?;
    info!("user: {:#?}", user);
    assert!(user.is_some());
    Ok(())
}

async fn mock_str() -> Result<(), DbErr> {
    // 创建表并返回pool
    let sql = r#"CREATE TABLE `user` 
    (
        `id` INT NULL,
        `name` TEXT NOT NULL,
        `status` BOOLEAN NOT NULL,
        PRIMARY KEY (`id`)
    );"#;
    let pool = Mock::builder().await?.migration_str(sql).await?.build();

    // 插入数据
    let sql = r#"INSERT INTO user (id,name, status)
     VALUES (1,'zhangsan',true);"#;
    pool.db().execute_unprepared(sql).await?;

    // 查看数据
    let stmt = Statement::from_sql_and_values(DbBackend::Sqlite, r#"SELECT * FROM  user"#, []);
    let result = pool.db().query_one(stmt).await?;
    info!("result: {:#?}", result);
    assert!(result.is_some());

    // 获取数据
    let user = User::find().one(pool.db()).await?;
    info!("user: {:#?}", user);
    assert!(user.is_some());

    Ok(())
}

// cargo test --package database --test mock_test -- --nocapture
#[tokio::test]
async fn main() -> Result<(), DbErr> {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("RUST_BACKTRACE", "1") };

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    mock_pool().await?;
    mock_entity().await?;
    mock_str().await?;

    Ok(())
}

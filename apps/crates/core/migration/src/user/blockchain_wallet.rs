//! 用户区块链钱包表
//! Entity: [`entity::user::BlockchainWallet`]

use sea_orm::{ConnectionTrait, DatabaseBackend, DeriveMigrationName};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match manager.get_database_backend() {
            DatabaseBackend::MySql => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_blockchain_wallet` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '钱包ID',
                        `user_id` INT(10) UNIQUE NOT NULL COMMENT '用户ID',
                        `wallet_address` VARCHAR(255) UNIQUE NOT NULL COMMENT '钱包地址',
                        `mnemonic` VARCHAR(255) NULL DEFAULT '' COMMENT '助记词',
                        `private_key` VARCHAR(255) NULL DEFAULT '' COMMENT '私钥',
                        `chain_id` INT(10) NULL DEFAULT 0 COMMENT '区块链ID',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '用户区块链钱包表';
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS 
                    "t_user_blockchain_wallet" (
                        "id" SERIAL PRIMARY KEY,
                        "user_id" INT UNIQUE NOT NULL,
                        "wallet_address" VARCHAR(255) UNIQUE NOT NULL,
                        "mnemonic" VARCHAR(255) DEFAULT '',
                        "private_key" VARCHAR(255) DEFAULT '',
                        "chain_id" INT DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "created_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    COMMENT ON TABLE t_user_blockchain_wallet IS '用户区块链钱包表';
                    COMMENT ON COLUMN t_user_blockchain_wallet.id IS '钱包ID';
                    COMMENT ON COLUMN t_user_blockchain_wallet.user_id IS '用户ID';
                    COMMENT ON COLUMN t_user_blockchain_wallet.wallet_address IS '钱包地址';
                    COMMENT ON COLUMN t_user_blockchain_wallet.mnemonic IS '助记词';
                    COMMENT ON COLUMN t_user_blockchain_wallet.private_key IS '私钥';
                    COMMENT ON COLUMN t_user_blockchain_wallet.chain_id IS '区块链ID';
                    COMMENT ON COLUMN t_user_blockchain_wallet.desc IS '描述信息';
                    COMMENT ON COLUMN t_user_blockchain_wallet.created_at IS '创建时间';
                    COMMENT ON COLUMN t_user_blockchain_wallet.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_user_blockchain_wallet` (  -- 用户区块链钱包表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 钱包ID
                        `user_id` INTEGER UNIQUE NOT NULL, -- 用户ID
                        `wallet_address` TEXT UNIQUE NOT NULL, -- 钱包地址
                        `mnemonic` TEXT DEFAULT '', -- 助记词
                        `private_key` TEXT DEFAULT '', -- 私钥
                        `chain_id` INTEGER DEFAULT 0, -- 区块链ID
                        `desc` TEXT DEFAULT '', -- 描述信息
                        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );
                    ",
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `t_user_blockchain_wallet`")
            .await?;

        Ok(())
    }
}

//! 用户区块链钱包表
//! Entity: [`entity::user::UserBlockchainWallet`]

use sea_orm::{
    DeriveIden, DeriveMigrationName,
    sea_query::{ColumnDef, Expr, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(UserBlockchainWallet::Table)
                    .comment("用户区块链钱包表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserBlockchainWallet::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("钱包ID"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::WalletAddress)
                            .string()
                            .string_len(255)
                            .unique_key()
                            .not_null()
                            .comment("钱包地址"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::Mnemonic)
                            .string()
                            .string_len(255)
                            .null()
                            .comment("助记词"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::PrivateKey)
                            .string()
                            .string_len(255)
                            .null()
                            .comment("私钥"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::ChainId)
                            .integer()
                            .null()
                            .comment("区块链ID"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::Desc)
                            .string()
                            .string_len(200)
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(UserBlockchainWallet::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(UserBlockchainWallet::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserBlockchainWallet {
    #[sea_orm(iden = "t_user_blockchain_wallet")]
    Table,
    Id,
    UserId,
    WalletAddress,
    Mnemonic,
    PrivateKey,
    ChainId,
    Desc,
    CreatedAt,
    UpdatedAt,
}

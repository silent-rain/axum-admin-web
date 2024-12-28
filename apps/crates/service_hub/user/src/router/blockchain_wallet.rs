//! 用户区块链钱包管理

use crate::controller::blockchain_wallet::BlockchainWalletController;

use axum::{routing::get, Router};

/// 路由器
pub struct BlockchainWalletRouter;

impl BlockchainWalletRouter {
    /// 注册`用户区块链钱包管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/blockchain-wallets",
            Router::new()
                .route(
                    "/",
                    get(BlockchainWalletController::list).post(BlockchainWalletController::create),
                )
                .route(
                    "/:id",
                    get(BlockchainWalletController::info)
                        .put(BlockchainWalletController::update)
                        .delete(BlockchainWalletController::delete),
                ),
        )
    }
}

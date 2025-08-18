//! 用户区块链钱包管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::blockchain_wallet::{
        CreateBlockchainWalletReq, CreateBlockchainWalletResp, DeleteBlockchainWalletReq,
        DeleteBlockchainWalletResp, GetBlockchainWalletReq, GetBlockchainWalletResp,
        GetBlockchainWalletsReq, GetBlockchainWalletsResp, UpdateBlockchainWalletReq,
        UpdateBlockchainWalletResp,
    },
    service::blockchain_wallet::BlockchainWalletService,
};

/// 控制器
pub struct BlockchainWalletController;

impl BlockchainWalletController {
    /// 获取用户区块链钱包列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetBlockchainWalletsReq>,
    ) -> Responder<GetBlockchainWalletsResp> {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let (results, total) = blockchain_wallet_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取用户区块链钱包信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetBlockchainWalletReq>,
    ) -> Responder<GetBlockchainWalletResp> {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let result = blockchain_wallet_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加用户区块链钱包
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(data): Json<CreateBlockchainWalletReq>,
    ) -> Responder<CreateBlockchainWalletResp> {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let _result = blockchain_wallet_service.create(data).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新用户区块链钱包
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateBlockchainWalletReq>,
    ) -> Responder<UpdateBlockchainWalletResp> {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let _result = blockchain_wallet_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除用户区块链钱包
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteBlockchainWalletReq>,
    ) -> Responder<DeleteBlockchainWalletResp> {
        let blockchain_wallet_service: BlockchainWalletService = provider.provide();
        let _result = blockchain_wallet_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}

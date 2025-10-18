//! 用户区块链钱包管理

use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::blockchain_wallet;

/// 查询用户区块链钱包列表 请求体
#[derive(Default, Deserialize, Validate)]
pub struct GetBlockchainWalletsReq {
    /// 当前分页
    pub page: u64,
    /// 页面大小
    pub page_size: u64,
    /// 开始时间
    pub start_time: Option<String>,
    /// 结束时间
    pub end_time: Option<String>,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 钱包地址
    pub wallet_address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockchainWalletsResp {
    pub data_list: Vec<blockchain_wallet::Model>,
    pub total: u64,
}

/// 查询用户区块链钱包信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetBlockchainWalletReq {
    /// 钱包ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBlockchainWalletResp {
    #[serde(flatten)]
    data: blockchain_wallet::Model,
}

/// 添加用户区块链钱包 请求体
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateBlockchainWalletReq {
    /// 用户ID
    pub user_id: i32,
    /// 钱包地址
    pub wallet_address: String,
    /// 助记词
    pub mnemonic: Option<String>,
    /// 私钥
    pub private_key: Option<String>,
    /// 区块链ID
    pub chain_id: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateBlockchainWalletResp {}

/// 更新用户区块链钱包数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateBlockchainWalletReq {
    /// 钱包ID
    pub id: i32,
    /// 区块链ID
    pub chain_id: Option<i32>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBlockchainWalletResp {}

/// 删除用户区块链钱包 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteBlockchainWalletReq {
    /// 钱包ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteBlockchainWalletResp {}

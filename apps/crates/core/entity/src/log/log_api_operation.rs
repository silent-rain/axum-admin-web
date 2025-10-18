//! API操作日志表

use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey,
    DeriveRelation, EnumIter, PrimaryKeyTrait,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

/// API操作日志表
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_log_api_operation")]
pub struct Model {
    /// 自增ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: Option<i32>,
    /// 用户名称
    pub username: Option<String>,
    /// 请求ID
    pub request_id: Option<String>,
    /// 请求状态码
    pub status_code: i32,
    /// 请求方法
    pub method: String,
    /// 请求地址路径
    pub path: String,
    /// Content-Type
    pub content_type: String,
    /// 请求参数
    pub query: Option<String>,
    /// 请求体/响应体
    #[sea_orm(column_type = "custom(\"LONGTEXT\")", nullable)]
    pub body: Option<String>,
    /// 请求IP
    pub remote_addr: String,
    /// 用户代理
    pub user_agent: String,
    /// 耗时,毫秒
    pub cost: i16,
    /// 请求类型:REQ/RESP
    pub http_type: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        Ok(self)
    }
}

pub mod enums {
    use super::*;

    /// Api 操作日志类型
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum HttpType {
        /// 请求
        #[serde(rename = "REQ")]
        Req,
        /// 响应
        #[serde(rename = "RESP")]
        Resp,
    }

    impl From<HttpType> for String {
        fn from(value: HttpType) -> Self {
            match value {
                HttpType::Req => "REQ".to_owned(),
                HttpType::Resp => "RESP".to_owned(),
            }
        }
    }
}

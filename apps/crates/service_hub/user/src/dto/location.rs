//! 用户地理位置管理

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use validator::Validate;

use entity::user::location;

/// 查询用户地理位置列表
#[derive(Default, Deserialize, Validate)]
pub struct GetLocationsReq {
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLocationsResp {
    pub data_list: Vec<location::Model>,
    pub total: u64,
}

/// 查询用户地理位置信息 请求体
#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct GetLocationReq {
    /// 位置ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetLocationResp {
    #[serde(flatten)]
    data: location::Model,
}

/// 添加用户地理位置
#[derive(Serialize, Deserialize, Validate)]
pub struct CreateLocationReq {
    /// 用户ID
    pub user_id: i32,
    /// 省份
    pub province: String,
    /// 城市
    pub city: String,
    /// 区/县
    pub district: String,
    /// 详细地址
    pub address: String,
    /// 邮政编码
    pub postal_code: Option<String>,
    /// 经度
    pub longitude: Option<Decimal>,
    /// 纬度
    pub latitude: Option<Decimal>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLocationResp {}

/// 更新数据 请求体
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Validate)]
pub struct UpdateLocationReq {
    /// 位置ID
    pub id: i32,
    /// 省份
    pub province: String,
    /// 城市
    pub city: String,
    /// 区/县
    pub district: String,
    /// 详细地址
    pub address: String,
    /// 邮政编码
    pub postal_code: Option<String>,
    /// 经度
    pub longitude: Option<Decimal>,
    /// 纬度
    pub latitude: Option<Decimal>,
    /// 描述信息
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocationResp {}

/// 删除用户地理位置 请求体
#[derive(Debug, Default, Deserialize, Validate)]
pub struct DeleteLocationReq {
    /// 位置ID
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteLocationResp {}

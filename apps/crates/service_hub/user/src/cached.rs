//! 缓存
use core::time;

use cache::Cache;
use err_code::Error;

/// 用户接口访问权限KEY
pub const USER_OPENAPI_USER_ID_METHOD_PATH: &str = "USER_OPENAPI_USER_ID_METHOD_PATH";

/// 用户接口权限缓存过期时间
pub const USER_EXPIRY: u64 = 60 * 60 * 24;

/// 用户管理缓存
pub struct UserCached;

impl UserCached {
    /// 设置用户接口访问权限
    /// (user_id, path, method)
    pub async fn set_user_openapi_access_permission(user_id: i32, path: String, method: String) {
        Cache::default()
            .set_with_expiry(
                &format!(
                    "{}_{}_{}_{}",
                    USER_OPENAPI_USER_ID_METHOD_PATH, user_id, path, method
                ),
                true,
                time::Duration::from_secs(USER_EXPIRY),
            )
            .await;
    }
    /// 获取用户接口访问权限
    pub async fn get_user_openapi_access_permission(
        user_id: i32,
        path: String,
        method: String,
    ) -> Result<bool, Error> {
        let result = Cache::default()
            .get_with_expiry(&format!(
                "{}_{}_{}_{}",
                USER_OPENAPI_USER_ID_METHOD_PATH, user_id, path, method
            ))
            .await;
        let result = match result {
            Some(v) => v.value,
            None => return Err(Error::CacheNotFound),
        };
        let permission: bool =
            serde_json::from_value(result).map_err(|err| Error::JsonConvert(err.to_string()))?;
        Ok(permission)
    }
}

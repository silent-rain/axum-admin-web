//! 上下文管理

/// 接口鉴权类型
#[derive(Debug, Clone)]
pub enum ApiAuthType {
    System,
    Openapi,
}

/// 上下文模型
#[derive(Debug, Clone)]
pub struct Context {
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub user_name: String,
    /// 登陆日志ID
    pub user_login_id: i32,
    /// 接口请求UUID
    pub request_id: String,
    /// 接口鉴权类型
    pub api_auth_type: Option<ApiAuthType>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            user_id: 0,
            user_name: "".to_string(),
            user_login_id: 0,
            request_id: "".to_string(),
            api_auth_type: None,
        }
    }
}

/// 用户信息传递
impl Context {
    /// 获取用户ID
    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }
    /// 设置用户ID
    pub fn set_user_id(&mut self, user_id: i32) {
        self.user_id = user_id;
    }
    /// 获取用户昵称
    pub fn get_user_name(&self) -> String {
        self.user_name.clone()
    }
    /// 设置用户昵称
    pub fn set_user_name(&mut self, user_name: String) {
        self.user_name = user_name;
    }
    /// 获取登陆日志ID
    pub fn get_user_login_id(&self) -> i32 {
        self.user_login_id
    }
    /// 设置登陆日志ID
    pub fn set_user_login_id(&mut self, user_login_id: i32) {
        self.user_login_id = user_login_id;
    }

    /// 获取接口鉴权类型
    pub fn get_api_auth_type(&self) -> Option<ApiAuthType> {
        self.api_auth_type.clone()
    }
    /// 设置接口鉴权类型
    pub fn set_api_auth_type(&mut self, api_auth_type: ApiAuthType) {
        self.api_auth_type = Some(api_auth_type);
    }
}

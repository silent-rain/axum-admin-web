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
    /// 用户会话ID
    pub session_id: String,
    /// 用户ID
    pub user_id: i32,
    /// 用户名称
    pub user_name: String,
    /// 角色列表
    pub role_ids: Vec<i32>,
    /// 接口鉴权类型
    pub api_auth_type: Option<ApiAuthType>,
    /// 接口请求UUID
    pub request_id: String,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            session_id: "".to_string(),
            user_id: 0,
            user_name: "".to_string(),
            role_ids: vec![],
            request_id: "".to_string(),
            api_auth_type: None,
        }
    }
}

/// 用户信息传递
impl Context {
    /// 获取用户会话ID
    pub fn get_session_id(&self) -> String {
        self.session_id.clone()
    }
    /// 设置用户会话ID
    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = session_id;
    }

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
    /// 获取角色列表
    pub fn get_role_ids(&self) -> Vec<i32> {
        self.role_ids.clone()
    }
    /// 设置角色列表
    pub fn set_role_ids(&mut self, role_ids: Vec<i32>) {
        self.role_ids = role_ids;
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

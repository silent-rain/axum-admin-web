//!环境配置

use serde::{Deserialize, Serialize};

/// 环境配置 结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Env {
    pub env: String, // 环境名称: prod/stag/dev
}

impl Default for Env {
    fn default() -> Self {
        Self {
            env: "prod".to_string(),
        }
    }
}

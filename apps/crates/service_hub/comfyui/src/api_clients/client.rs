//! ComfyUI 接口
//! 参考文档
//! - https://blog.csdn.net/ddafei/article/details/140328897
//! - https://blog.csdn.net/jjs15259655776/article/details/134388940
//! - https://blog.csdn.net/jjs15259655776/article/details/134373096
//! - https://apifox.com/apidoc/shared-3c858595-4cba-47b3-b359-96509a049c68/websocket-3519713
//! - https://docs.infini-ai.com/gen-studio/api/tutorial-comfyui.html#%E7%AE%80%E4%BB%8B

use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue};

pub use super::error::Error;

pub struct ComfyUIClient {
    pub client: reqwest::Client,
    pub headers: HeaderMap,
    pub timeout: Duration,
    pub base_api: String,
}

impl ComfyUIClient {
    pub fn new(base_api: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();

        ComfyUIClient {
            base_api,
            timeout: Duration::from_secs(5),
            client,
            headers,
        }
    }
}

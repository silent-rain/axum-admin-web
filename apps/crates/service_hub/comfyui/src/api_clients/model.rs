//! 模型相关接口

use super::{client::ComfyUIClient, error::Error};

impl ComfyUIClient {
    /// 获取连续的词向量文件列表
    pub async fn embeddings(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/embeddings", self.base_api);

        let resp = self
            .client
            .get(url)
            .timeout(self.timeout)
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_API: &str = "http://127.0.0.1:8188/api";

    #[tokio::test]
    async fn test_embeddings() {
        let client = ComfyUIClient::new(BASE_API.to_string());
        let results = client.embeddings().await;
        println!("results: {:#?}", results);
    }
}

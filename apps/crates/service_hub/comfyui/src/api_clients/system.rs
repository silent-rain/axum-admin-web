//! 系统相关接口

use super::{client::ComfyUIClient, dto::SystemStats, error::Error};

impl ComfyUIClient {
    /// 系统统计信息
    pub async fn system_stats(&self) -> Result<SystemStats, Error> {
        let url = format!("{}/system_stats", self.base_api);

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

    #[tokio::test]
    async fn test_system_stats() {
        let client = ComfyUIClient::new();
        let results = client.system_stats().await;
        println!("results: {:#?}", results);
    }
}

//! 节点相关接口

use serde_json::Value;

use super::{client::ComfyUIClient, error::Error};

impl ComfyUIClient {
    /// 获取节点信息
    /// node_class Some, 根据组件名称获取系统中组件信息
    /// node_class None, 获取所有所有组件信息
    pub async fn object_info(&self, node_class: Option<&str>) -> Result<Value, Error> {
        let url = match node_class {
            Some(v) => format!("{}/object_info/{}", self.base_api, v),
            None => format!("{}/object_info", self.base_api),
        };

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

    /// 获取扩展节点文件列表
    pub async fn extensions(&self) -> Result<Vec<String>, Error> {
        let url = format!("{}/extensions", self.base_api);

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

    /// TODO 待定
    /// 预览元数据
    pub async fn view_metadata(&self) -> Result<Value, Error> {
        let url = format!("{}/view_metadata", self.base_api);

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
    async fn test_object_info() {
        let client = ComfyUIClient::new();
        let results = client.object_info(None).await;
        println!("results: {:#?}", results);

        let result = client.object_info(Some("KSampler")).await;
        println!("KSampler result: {:#?}", result);
    }

    #[tokio::test]
    async fn test_extensions() {
        let client = ComfyUIClient::new();
        let results = client.extensions().await;
        println!("results: {:#?}", results);
    }

    #[tokio::test]
    async fn test_view_metadata() {
        let client = ComfyUIClient::new();
        let results = client.view_metadata().await;
        println!("results: {:#?}", results);
    }
}

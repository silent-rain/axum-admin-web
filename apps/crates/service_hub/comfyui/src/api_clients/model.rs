//! 模型相关接口

use super::{
    client::ComfyUIClient,
    dto::{Embedding, Model},
    error::Error,
};

impl ComfyUIClient {
    /// 获取连续的词向量模型列表
    pub async fn old_embeddings(&self) -> Result<Vec<String>, Error> {
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

    /// 获取连续的词向量模型列表
    /// - experiment
    pub async fn embeddings(&self) -> Result<Vec<Embedding>, Error> {
        let url = format!("{}/experiment/models/embeddings", self.base_api);

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

    /// 获取 Checkpoint 模型列表
    /// - experiment
    pub async fn checkpoints(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/checkpoints", self.base_api);

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

    /// 获取 lora 模型列表
    /// - experiment
    pub async fn loras(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/loras", self.base_api);

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

    /// 获取 Vae 模型列表
    /// - experiment
    pub async fn vaes(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/vae", self.base_api);

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

    /// 获取 clip_vision 模型列表
    /// - experiment
    pub async fn clip_visions(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/clip_vision", self.base_api);

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

    /// 获取 controlnet 模型列表
    /// - experiment
    pub async fn controlnets(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/controlnet", self.base_api);

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

    /// 获取 upscale_model 模型列表
    /// - experiment
    pub async fn upscale_models(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/upscale_models", self.base_api);

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

    /// 获取 ipadapter 模型列表
    /// - experiment
    pub async fn ipadapters(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/ipadapter", self.base_api);

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

    /// 获取 unet_gguf 模型列表
    /// - experiment
    pub async fn unet_ggufs(&self) -> Result<Vec<Model>, Error> {
        let url = format!("{}/experiment/models/unet_gguf", self.base_api);

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

    #[ignore]
    #[tokio::test]
    async fn test_embeddings() {
        let client = ComfyUIClient::new();
        let results = client.embeddings().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_checkpoints() {
        let client = ComfyUIClient::new();
        let results = client.checkpoints().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_loras() {
        let client = ComfyUIClient::new();
        let results = client.loras().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_vaes() {
        let client = ComfyUIClient::new();
        let results = client.vaes().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_clip_visions() {
        let client = ComfyUIClient::new();
        let results = client.clip_visions().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_controlnets() {
        let client = ComfyUIClient::new();
        let results = client.controlnets().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_upscale_models() {
        let client = ComfyUIClient::new();
        let results = client.upscale_models().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_ipadapters() {
        let client = ComfyUIClient::new();
        let results = client.ipadapters().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_unet_ggufs() {
        let client = ComfyUIClient::new();
        let results = client.unet_ggufs().await;
        println!("results: {:#?}", results);
    }
}

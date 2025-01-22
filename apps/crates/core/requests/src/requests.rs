//! reqwest 请求库
use std::{collections::HashMap, path::Path, str::FromStr, time::Duration};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    multipart::{Form, Part},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::Error;

/// 客户端
#[derive(Default)]
pub struct Client {
    timeout: Duration,
    headers: HeaderMap,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = reqwest::Client::new();
        Client {
            timeout: Duration::from_secs(5),
            client,
            headers,
        }
    }

    /// set http headers
    pub fn headers(mut self, headers: HashMap<String, String>) -> Result<Self, Error> {
        for (k, v) in headers {
            self.headers.insert(
                HeaderName::from_str(&k).map_err(Error::InvalidHeaderName)?,
                HeaderValue::from_str(&v).map_err(Error::InvalidHeaderValue)?,
            );
        }

        Ok(self)
    }

    /// set http timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// get requests
    pub async fn get<Req, Resp>(self, url: &str, params: Req) -> Result<Resp, Error>
    where
        Req: Serialize,
        (): Into<Req>,
        Resp: for<'a> Deserialize<'a>,
    {
        let resp = self
            .client
            .get(url)
            .query(&params)
            .send()
            .await?
            .json::<Resp>()
            .await?;

        Ok(resp)
    }

    /// post requests
    pub async fn post<Req, Resp>(self, url: &str, payload: Req) -> Result<Resp, Error>
    where
        Req: serde::Serialize,
        Resp: for<'de> serde::de::Deserialize<'de>,
    {
        let resp = self
            .client
            .post(url)
            .headers(self.headers)
            .timeout(self.timeout) // 设置超时时间
            .json(&payload)
            .send()
            .await?
            .json::<Resp>()
            .await?;

        Ok(resp)
    }

    /// form 表单
    pub async fn form<Req, Resp>(self, url: &str, params: Req) -> Result<Resp, Error>
    where
        Req: serde::Serialize,
        Resp: for<'de> serde::de::Deserialize<'de>,
    {
        // 表单 对应的Content-Type
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        let resp = self
            .client
            .get(url)
            .headers(self.headers)
            .headers(headers)
            .timeout(self.timeout) // 设置超时时间
            .form(&params)
            .send()
            .await?
            .json::<Resp>()
            .await?;

        Ok(resp)
    }

    /// 上传文件
    pub async fn upload<Resp>(self, url: &str, file_path: String) -> Result<Resp, Error>
    where
        Resp: for<'de> serde::de::Deserialize<'de>,
    {
        // 文件处理
        let path = Path::new(&file_path);
        let extension = path
            .extension()
            .ok_or_else(|| Error::FileNameExtension("not found extension".to_string()))?
            .to_str()
            .ok_or_else(|| Error::ConvertType("OsStr convert str failed".to_string()))?;
        let file_bytes = tokio::fs::read(path).await?;
        let file_name = Uuid::new_v4().to_string();

        let part = Part::stream(file_bytes.clone()).file_name(format!("{file_name}.{extension}"));
        let form: Form = Form::new().part("image", part);

        let resp = self
            .client
            .post(url)
            .timeout(self.timeout)
            .multipart(form)
            .send()
            .await?
            .json::<Resp>()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get() -> anyhow::Result<()> {
        #[allow(unused)]
        #[derive(Debug, Deserialize)]
        struct Response {
            pub origin: String,
        }

        let headers = HashMap::new();
        let result = Client::new()
            .headers(headers)?
            .timeout(Duration::from_secs(5))
            .get::<(), Response>("https://httpbin.org/ip", ())
            .await;

        assert!(result.is_ok());
        // println!("result: {:?}", result?.origin);

        Ok(())
    }

    #[tokio::test]
    async fn test_post() -> anyhow::Result<()> {
        #[allow(unused)]
        #[derive(Debug, Deserialize)]
        struct Response {
            pub origin: String,
            pub url: String,
        }

        let headers = HashMap::new();
        let result = Client::new()
            .headers(headers)?
            .timeout(Duration::from_secs(5))
            .post::<(), Response>("http://httpbin.org/post", ())
            .await;

        assert!(result.is_ok());
        // println!("result: {:?}", result?.origin);
        Ok(())
    }
}

//! 任务相关接口

use std::collections::HashMap;

use reqwest::StatusCode;

use super::{
    client::ComfyUIClient,
    dto::{History, PromptReq, PromptResult, QueueRemaining, Queues},
    error::Error,
};

impl ComfyUIClient {
    // TODO ws 连接websocket实时获取绘图进度消息
    // ws://127.0.0.1:8188/ws

    /// 发布绘图任务
    /// 此接口只做任务下发，返回任务ID信息。
    pub async fn prompt(&self, payload: PromptReq) -> Result<PromptResult, Error> {
        let url = format!("{}/prompt", self.base_api);

        let resp = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .json(&payload)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// 获取服务器当前剩余任务列队的数量
    pub async fn queue_remaining(&self) -> Result<QueueRemaining, Error> {
        let url = format!("{}/prompt", self.base_api);

        let resp = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// 获取所有历史任务数据
    /// prompt_id： Some, 根据任务id获取历史数据
    /// prompt_id： None, 获取所有历史任务数据
    pub async fn historys(
        &self,
        max_items: Option<i32>,
    ) -> Result<HashMap<String, History>, Error> {
        let max_items = max_items.map_or(64, |v| v);
        let url = format!("{}/history?max_items={}", self.base_api, max_items);

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

    /// 获取指定历史任务数据
    pub async fn history(&self, prompt_id: &str) -> Result<HashMap<String, History>, Error> {
        let url = format!("{}/history/{}", self.base_api, prompt_id);

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

    /// 获取所有的队列
    ///
    /// 获取详细任务队列信息，正在运行的以及挂起的
    pub async fn queues(&self) -> Result<Queues, Error> {
        let url = format!("{}/queue", self.base_api);

        let resp = self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// 清空队列
    /// 清除所有等待的列队, 执行中的任务无法清除，无返回信息则为成功.
    pub async fn clear_queue(&self) -> Result<bool, Error> {
        let url = format!("{}/queue", self.base_api);

        let mut payload = HashMap::new();
        payload.insert("clear", true);

        let status = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .json(&payload)
            .send()
            .await?
            .status();

        Ok(status.as_u16() == StatusCode::OK)
    }

    /// 删除队列
    /// 根据prompt_id列表删除指定的队列
    pub async fn delete_queue(&self, prompt_ids: Vec<String>) -> Result<bool, Error> {
        let url = format!("{}/queue", self.base_api);

        let mut payload = HashMap::new();
        payload.insert("delete", prompt_ids);

        let status = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .json(&payload)
            .send()
            .await?
            .status();

        Ok(status.as_u16() == StatusCode::OK)
    }

    /// 取消当前任务
    pub async fn interrupt(&self) -> Result<bool, Error> {
        let url = format!("{}/interrupt", self.base_api);

        let status = self
            .client
            .post(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .status();

        Ok(status.as_u16() == StatusCode::OK)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use serde_json::json;
    use uuid::Uuid;

    use super::*;

    #[ignore]
    #[tokio::test]
    async fn test_prompt() {
        let prompt = json!(
            {
                "14": {
                  "inputs": {
                    "image": "3909.jpg_wh860.jpg",
                    "upload": "image"
                  },
                  "class_type": "LoadImage",
                  "_meta": {
                    "title": "Load Image"
                  }
                },
                "15": {
                  "inputs": {
                    "images": [
                      "14",
                      0
                    ]
                  },
                  "class_type": "PreviewImage",
                  "_meta": {
                    "title": "Preview Image"
                  }
                }
            }
        );

        let client_id = Uuid::new_v4().to_string().replace("-", "");
        let payload = PromptReq { prompt, client_id };

        let client = ComfyUIClient::new();
        let results = client.prompt(payload).await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_queue_remaining() {
        let client = ComfyUIClient::new();
        let results = client.queue_remaining().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_historys() {
        let client = ComfyUIClient::new();

        let results = client.historys(Some(64)).await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_history() {
        let client = ComfyUIClient::new();
        let result = client.history("6ee684f7-29e3-4b35-a19c-579412ef8495").await;
        println!("result: {:#?}", result);
    }

    #[ignore]
    #[tokio::test]
    async fn test_queue_list() {
        let client = ComfyUIClient::new();
        let results = client.queues().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_clear_queue() {
        let client = ComfyUIClient::new();

        let results = client.clear_queue().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_delete_queue() {
        let client = ComfyUIClient::new();

        // 删除指定队列
        let prompt_ids = vec!["3260b56f-755b-4c17-af12-c5ad1c72c2c7".to_string()];
        let results = client.delete_queue(prompt_ids).await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_interrupt() {
        let client = ComfyUIClient::new();
        let results = client.interrupt().await;
        println!("results: {:#?}", results);
    }

    #[ignore]
    #[tokio::test]
    async fn test_queue_date_parser() -> anyhow::Result<()> {
        let data = {
            r#"
        {
    "queue_running": [
        [
            0,
            "b3309862-8907-4816-b970-5cdc34627b89",
            {
                
                "8": {
                    "inputs": {
                        "samples": [
                            "3",
                            0
                        ],
                        "vae": [
                            "4",
                            2
                        ]
                    },
                    "class_type": "VAEDecode",
                    "_meta": {
                        "title": "VAE Decode"
                    }
                },
                "9": {
                    "inputs": {
                        "filename_prefix": "ComfyUI",
                        "images": [
                            "8",
                            0
                        ]
                    },
                    "class_type": "SaveImage",
                    "_meta": {
                        "title": "Save Image"
                    }
                },
                "10": {
                    "inputs": {
                        "images": [
                            "8",
                            0
                        ]
                    },
                    "class_type": "PreviewImage",
                    "_meta": {
                        "title": "Preview Image"
                    }
                }
            },
            {
                "extra_pnginfo": {
                    "workflow": {
                        "last_node_id": 10,
                        "last_link_id": 10,
                        "groups": [],
                        "config": {},
                        "extra": {
                            "ds": {
                                "scale": 0.7247295000000005,
                                "offset": [
                                    33.73867572144238,
                                    -137.64794105938893
                                ]
                            },
                            "ue_links": []
                        },
                        "version": 0.4,
                        "widget_idx_map": {
                            "3": {
                                "seed": 0,
                                "sampler_name": 4,
                                "scheduler": 5
                            }
                        },
                        "seed_widgets": {
                            "3": 0
                        }
                    }
                },
                "client_id": ""
            },
            [
                "9",
                "10"
            ]
        ]
    ],
    "queue_pending": [
        [
            1,
            "202de9b0-4a86-44b9-bc33-b9482efe7df6",
            {
                "7": {
                    "inputs": {
                        "text": "text, watermark",
                        "clip": [
                            "4",
                            1
                        ]
                    },
                    "class_type": "CLIPTextEncode",
                    "_meta": {
                        "title": "CLIP Text Encode (Prompt)"
                    }
                },
                "8": {
                    "inputs": {
                        "samples": [
                            "3",
                            0
                        ],
                        "vae": [
                            "4",
                            2
                        ]
                    },
                    "class_type": "VAEDecode",
                    "_meta": {
                        "title": "VAE Decode"
                    }
                }
            },
            {
                "extra_pnginfo": {
                    "workflow": {
                        "last_node_id": 10,
                        "last_link_id": 10,
                        "groups": [],
                        "config": {},
                        "extra": {
                            "ds": {
                                "scale": 0.7247295000000005,
                                "offset": [
                                    33.73867572144238,
                                    -137.64794105938893
                                ]
                            },
                            "ue_links": []
                        },
                        "version": 0.4,
                        "widget_idx_map": {
                            "3": {
                                "seed": 0,
                                "sampler_name": 4,
                                "scheduler": 5
                            }
                        },
                        "seed_widgets": {
                            "3": 0
                        }
                    }
                },
                "client_id": ""
            },
            [
                "9",
                "10"
            ]
        ],
        [
            2,
            "e4f50e20-7fe3-42da-ba17-1aad3e9f2c0b",
            {
                "9": {
                    "inputs": {
                        "filename_prefix": "ComfyUI",
                        "images": [
                            "8",
                            0
                        ]
                    },
                    "class_type": "SaveImage",
                    "_meta": {
                        "title": "Save Image"
                    }
                }
            },
            {
                "extra_pnginfo": {
                    "workflow": {
                        "last_node_id": 10,
                        "last_link_id": 10,
                        "groups": [],
                        "config": {},
                        "extra": {
                            "ds": {
                                "scale": 0.7247295000000005,
                                "offset": [
                                    33.73867572144238,
                                    -137.64794105938893
                                ]
                            },
                            "ue_links": []
                        },
                        "version": 0.4,
                        "widget_idx_map": {
                            "3": {
                                "seed": 0,
                                "sampler_name": 4,
                                "scheduler": 5
                            }
                        },
                        "seed_widgets": {
                            "3": 0
                        }
                    }
                },
                "client_id": ""
            },
            [
                "9",
                "10"
            ]
        ]
    ]
}
        "#
        };
        let target_prompt_id = "e4f50e20-7fe3-42da-ba17-1aad3e9f2c0b";

        let data: Queues = serde_json::from_str(data)?;
        let queue_pending_total = data.queue_pending.len();
        println!("queue_pending_total: {:#?}", queue_pending_total);

        let queue_running_prompt_id = data.queue_running[0].get(1).and_then(|v| v.as_str());
        println!("queue_running_prompt_id: {:#?}", queue_running_prompt_id);

        let queue_pending_prompt_id: Vec<String> = data
            .queue_pending
            .clone()
            .into_iter()
            .filter(|v| {
                let prompt_id = v.get(1).and_then(|v| v.as_str());
                Some(target_prompt_id) == prompt_id
            })
            .filter_map(|v| {
                let prompt_id = v.get(1).and_then(|v| v.as_str());
                prompt_id.map(|v| v.to_string())
            })
            .collect();
        println!("queue_pending_prompt_id: {:#?}", queue_pending_prompt_id);

        // 查找 target_prompt_id 所在的索引
        let queue_pending_index = data.queue_pending.iter().position(|v| {
            let prompt_id = v.get(1).and_then(|v| v.as_str());
            Some(target_prompt_id) == prompt_id
        });
        println!("queue_pending_index: {:#?}", queue_pending_index);
        Ok(())
    }
}

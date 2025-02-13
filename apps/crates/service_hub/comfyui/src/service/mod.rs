//! 服务层

use nject::injectable;
pub mod task;

/*
全局配置:
- base_api
- 超时时间
- 模型列表
- lora列表

过程数据是否入库呢? 便于回溯? 服务器端,重启服务后历史信息会丢失
*/

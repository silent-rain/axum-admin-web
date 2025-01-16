# MySql使用文档

## Mysql 速查表

```sql
-- 创建数据库 
CREATE DATABASE IF NOT EXISTS `axum_admin_web` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

-- 修改表名
ALTER TABLE avatar
RENAME user_avatar;

-- 修改字段名称
ALTER TABLE token
CHANGE user_id `user_id` INT NOT NULL COMMENT '用户ID';

-- 添加字段
ALTER TABLE http_log
ADD `trace_id` VARCHAR(32) NULL COMMENT '请求traceId';

-- 添加唯一约束
ALTER TABLE okx_order.okx_main_order
ADD CONSTRAINT okx_main_order_uni_api_key UNIQUE KEY (api_key);

-- 添加主键约束
ALTER TABLE okx_order.minor_order_trade_history
ADD CONSTRAINT minor_order_trade_history_PK PRIMARY KEY (user_id, minor_order_id);

-- 删除字段
ALTER TABLE 数据表名
DROP 字段名;
```

## 备份与恢复

```sql
-- 备份数据库
mysqldump -uxxx -pxxx axum_admin_web > axum_admin_web.sql

mysqldump --skip-opt  -h127.0.0.1 -P3306 -uone -p --single-transaction --default-character-set=utf8 -q axum_admin_web user_log  > ./user_log-bak_20250103.sql

-- 恢复数据库
mysql -uxxx -pxxx axum_admin_web < axum_admin_web.sql
```

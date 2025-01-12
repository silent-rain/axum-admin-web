/*任务调度相关*/
-- 任务调度作业表
CREATE TABLE IF NOT EXISTS
    `t_schedule_job` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `name` VARCHAR(200) UNIQUE NOT NULL COMMENT '任务名称',
        `source` TINYINT(1) NOT NULL COMMENT '任务来源(0:用户定义,1:系统内部)',
        `job_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务类型(0:定时任务,1:即时任务)',
        `sys_code` VARCHAR(200) NOT NULL COMMENT '系统任务编码',
        `expression` VARCHAR(100) DEFAULT '' COMMENT 'cron表达式',
        `interval` INT DEFAULT 0 COMMENT '间隔时间,秒',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '任务状态(0:下线,1:上线)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '任务调度作业表';

CREATE INDEX idx_sys_code ON t_schedule_job (`sys_code`);

-- 任务调度状态日志表
CREATE TABLE IF NOT EXISTS
    `t_schedule_status_log` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '状态日志ID',
        `job_id` INT NOT NULL COMMENT '任务ID',
        `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
        `error` TEXT COMMENT '失败信息',
        `cost` INT NOT NULL COMMENT '耗时,毫秒',
        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '任务调度状态日志表';

CREATE INDEX idx_job_id ON t_schedule_status_log (`job_id`);

CREATE INDEX idx_uuid ON t_schedule_status_log (`uuid`);

-- 任务调度事件日志表
CREATE TABLE IF NOT EXISTS
    `t_schedule_event_log` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '事件日志ID',
        `job_id` INT NOT NULL COMMENT '任务ID',
        `uuid` VARCHAR(50) NOT NULL COMMENT '任务调度ID',
        `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '任务状态(0:开始,1:完成,2:停止,3:移除)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP() COMMENT '创建时间',
        PRIMARY KEY (`id`) USING BTREE
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '任务调度事件日志表';

CREATE INDEX idx_job_id ON t_schedule_event_log (`job_id`);

CREATE INDEX idx_uuid ON t_schedule_event_log (`uuid`);
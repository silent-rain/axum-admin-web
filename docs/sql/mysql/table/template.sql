-- 应用模板表
CREATE TABLE IF NOT EXISTS
    `t_app_template` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '模板ID',
        `user_id` INT NOT NULL COMMENT '用户ID',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '应用模板表';

CREATE INDEX idx_user_id ON t_app_template (`user_id`);
/*系统相关表*/
-- 图片验证码表
CREATE TABLE IF NOT EXISTS
    `t_sys_image_captcha` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `captcha_id` VARCHAR(40) UNIQUE NOT NULL COMMENT '验证码ID',
        `captcha` VARCHAR(10) NOT NULL COMMENT '验证码',
        `data` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
        `expire` INT NOT NULL DEFAULT 1 COMMENT '过期时间,秒',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:失效,true:有效)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '图片图片验证码表';

CREATE INDEX idx_captcha_id ON t_sys_image_captcha (`captcha_id`);

CREATE INDEX idx_status ON t_sys_image_captcha (`status`);

-- 配置表
CREATE TABLE IF NOT EXISTS
    `t_sys_config` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '配置ID',
        `pid` INT DEFAULT 0 COMMENT '父节点ID',
        `name` VARCHAR(64) NOT NULL COMMENT '配置名称',
        `code` VARCHAR(64) UNIQUE NOT NULL COMMENT '配置编码(英文)',
        `value` TEXT NULL COMMENT '配置值',
        `sort` INT NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) DEFAULT '' COMMENT '配置描述',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COLLATE = utf8mb4_unicode_ci COMMENT '配置表';

CREATE INDEX idx_pid ON t_sys_config (`pid`);

CREATE INDEX idx_name ON t_sys_config (`name`);

CREATE INDEX idx_code ON t_sys_config (`code`);

-- 图片资源表
CREATE TABLE IF NOT EXISTS
    `t_sys_file_resource` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '图片ID',
        `file_name` VARCHAR(32) NOT NULL COMMENT '图片名称',
        `hash` VARCHAR(32) UNIQUE NOT NULL COMMENT '图片HASH值',
        `data` MEDIUMBLOB NOT NULL COMMENT '图片数据, Base64编码',
        `extension` VARCHAR(20) NOT NULL COMMENT '图片文件扩展名, 如svg, png',
        `content_type` VARCHAR(20) NOT NULL COMMENT '内容类型, text/html',
        `size` INT NOT NULL COMMENT '图片文件大小，单位为字节',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '图片资源表';

CREATE INDEX idx_t_sys_file_resource_file_name ON t_sys_file_resource (`file_name`);

CREATE INDEX idx_t_sys_file_resource_hash ON t_sys_file_resource (`hash`);

-- 字典维度表
CREATE TABLE IF NOT EXISTS
    `t_sys_dict_dimension` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '字典维度ID',
        `name` VARCHAR(64) UNIQUE NOT NULL COMMENT '字典维度名称',
        `code` VARCHAR(64) UNIQUE NOT NULL COMMENT '字典维度编码',
        `sort` INT NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '字典维度表';

CREATE INDEX idx_name ON t_sys_dict_dimension (`name`);

CREATE INDEX idx_code ON t_sys_dict_dimension (`code`);

-- 字典数据表
CREATE TABLE IF NOT EXISTS
    `t_sys_dict_data` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '字典项ID',
        `dim_id` INT NOT NULL COMMENT '字典维度ID',
        `label` VARCHAR(64) NOT NULL COMMENT '字典项标签',
        `value` TEXT NOT NULL COMMENT '字典项值',
        `sort` INT NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '字典数据表';

CREATE INDEX idx_dim_id ON t_sys_dict_data (`dim_id`);

CREATE INDEX idx_label ON t_sys_dict_data (`label`);
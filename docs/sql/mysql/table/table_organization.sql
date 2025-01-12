/*组织相关表*/
-- 部门表
CREATE TABLE IF NOT EXISTS
    `t_org_department` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '部门ID',
        `pid` BIGINT DEFAULT NULL DEFAULT 0 COMMENT '上级部门ID',
        `pids` VARCHAR(200) DEFAULT NULL DEFAULT '' COMMENT '所有上级部门ID, 用逗号分开',
        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '部门名称',
        `sort` INT NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '部门描述',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '部门表';

CREATE INDEX idx_pid ON t_org_department (`pid`);

CREATE INDEX idx_name ON t_org_department (`name`);

-- 部门角色关系表
CREATE TABLE IF NOT EXISTS
    `t_org_department_role_rel` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '自增ID',
        `department_id` INT NOT NULL COMMENT '部门ID',
        `role_id` INT NOT NULL COMMENT '角色ID',
        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '部门角色关系表';

CREATE UNIQUE INDEX uk_department_id_role_id ON t_org_department_role_rel (`department_id`, `role_id`);

-- 岗位表
CREATE TABLE IF NOT EXISTS
    `t_org_position` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '岗位ID',
        `name` VARCHAR(100) UNIQUE NOT NULL COMMENT '岗位名称',
        `sort` INT NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '岗位描述',
        `department_id` INT DEFAULT 0 COMMENT '所属部门ID',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '岗位表';

CREATE INDEX idx_name ON t_org_position (`name`);

CREATE INDEX idx_department_id ON t_org_position (`department_id`);

-- 职级表
CREATE TABLE IF NOT EXISTS
    `t_org_rank` (
        `id` INT AUTO_INCREMENT NOT NULL COMMENT '职级ID',
        `name` VARCHAR(20) UNIQUE NOT NULL COMMENT '职级名称',
        `level` INT UNIQUE NOT NULL COMMENT '职级等级',
        `sort` INT NULL DEFAULT 0 COMMENT '排序',
        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '职级描述',
        `status` BOOL NOT NULL DEFAULT TRUE COMMENT '状态(false:停用,true:正常)',
        `created_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
        `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
        PRIMARY KEY (`id`)
    ) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COMMENT = '职级表';
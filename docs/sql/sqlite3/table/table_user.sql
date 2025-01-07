/*用户与身份管理相关表*/
-- 角色表
CREATE TABLE IF NOT EXISTS
    t_user_role (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 角色ID
        "name" TEXT UNIQUE NOT NULL, -- 角色名称
        sort INTEGER DEFAULT 0, -- 排序
        "desc" TEXT DEFAULT '', -- 描述信息
        "status" INTEGER NOT NULL DEFAULT 1, -- 状态(0:停用,1:正常)
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE INDEX idx_name ON t_user_role ("name");

-- 用户信息表
CREATE TABLE IF NOT EXISTS
    t_user_base (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 用户ID，自增
        username VARCHAR(32) NOT NULL UNIQUE, -- 用户名，唯一
        real_name VARCHAR(32) DEFAULT '', -- 真实姓名
        gender INTEGER NOT NULL, -- 性别
        "password" VARCHAR(64) NOT NULL, -- 密码
        "status" INTEGER NOT NULL, -- 用户状态
        age INTEGER DEFAULT 0, -- 年龄
        date_birth VARCHAR(20) DEFAULT '', -- 出生日期
        avatar VARCHAR(200) DEFAULT '', -- 头像URL
        intro VARCHAR(200) DEFAULT '', -- 简介
        "desc" VARCHAR(200) DEFAULT '', -- 描述
        "address" VARCHAR(200) DEFAULT '', -- 地址
        share_code VARCHAR(16) DEFAULT '', -- 分享码
        preferences VARCHAR(200) DEFAULT '', -- 偏好
        department_id INTEGER DEFAULT 0, -- 部门ID
        position_id INTEGER DEFAULT 0, -- 职位ID
        rank_id INTEGER DEFAULT 0, -- 职级ID
        member_level_id INTEGER DEFAULT 0, -- 会员等级ID
        created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE INDEX idx_username ON t_user_base (username);

-- 用户邮箱表
CREATE TABLE IF NOT EXISTS
    t_user_email (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 邮箱ID
        user_id INTEGER UNIQUE NOT NULL, -- 用户ID
        email TEXT UNIQUE NOT NULL, -- 邮箱
        "desc" TEXT DEFAULT '', -- 描述信息
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE INDEX idx_user_id ON t_user_email (user_id);

-- SQLite does not support foreign key constraints with ON UPDATE CASCADE
PRAGMA foreign_keys = ON;

CREATE TRIGGER fk_user_email_user_id_update AFTER
UPDATE ON t_user_email FOR EACH ROW WHEN NEW.user_id != OLD.user_id BEGIN
UPDATE t_user_base
SET
    id = NEW.user_id
WHERE
    id = OLD.user_id;

END;

CREATE TRIGGER fk_user_email_user_id_delete BEFORE DELETE ON t_user_email FOR EACH ROW BEGIN
DELETE FROM t_user_base
WHERE
    id = OLD.user_id;

END;

-- 用户手机号表
CREATE TABLE IF NOT EXISTS
    t_user_phone (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 手机号ID
        user_id INTEGER UNIQUE NOT NULL, -- 用户ID
        phone TEXT UNIQUE NOT NULL, -- 手机号码
        "desc" TEXT DEFAULT '', -- 描述信息
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE INDEX idx_user_id ON t_user_phone (user_id);

-- SQLite does not support foreign key constraints with ON UPDATE CASCADE
PRAGMA foreign_keys = ON;

CREATE TRIGGER fk_user_phone_user_id_update AFTER
UPDATE ON t_user_phone FOR EACH ROW WHEN NEW.user_id != OLD.user_id BEGIN
UPDATE t_user_base
SET
    id = NEW.user_id
WHERE
    id = OLD.user_id;

END;

CREATE TRIGGER fk_user_phone_user_id_delete BEFORE DELETE ON t_user_phone FOR EACH ROW BEGIN
DELETE FROM t_user_base
WHERE
    id = OLD.user_id;

END;

-- 用户区块链钱包表
CREATE TABLE IF NOT EXISTS
    t_user_blockchain_wallet (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 钱包ID
        user_id INTEGER UNIQUE NOT NULL, -- 用户ID
        wallet_address TEXT UNIQUE NOT NULL, -- 钱包地址
        mnemonic TEXT DEFAULT '', -- 助记词
        private_key TEXT DEFAULT '', -- 私钥
        chain_id INTEGER DEFAULT 0, -- 区块链ID
        "desc" TEXT DEFAULT '', -- 描述信息
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE INDEX idx_user_id ON t_user_blockchain_wallet (user_id);

CREATE INDEX idx_wallet_address ON t_user_blockchain_wallet (wallet_address);

-- SQLite does not support foreign key constraints with ON UPDATE CASCADE
PRAGMA foreign_keys = ON;

CREATE TRIGGER fk_user_blockchain_wallet_user_id_update AFTER
UPDATE ON t_user_blockchain_wallet FOR EACH ROW WHEN NEW.user_id != OLD.user_id BEGIN
UPDATE t_user_base
SET
    id = NEW.user_id
WHERE
    id = OLD.user_id;

END;

CREATE TRIGGER fk_user_blockchain_wallet_user_id_delete BEFORE DELETE ON t_user_blockchain_wallet FOR EACH ROW BEGIN
DELETE FROM t_user_base
WHERE
    id = OLD.user_id;

END;

-- 用户角色关系表
CREATE TABLE IF NOT EXISTS
    t_user_role_rel (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 自增ID
        user_id INTEGER NOT NULL, -- 用户ID
        role_id INTEGER NOT NULL, -- 角色ID
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 创建时间
    );

CREATE UNIQUE INDEX uk_user_id_role_id ON t_user_role_rel (user_id, role_id);

-- SQLite does not support foreign key constraints with ON UPDATE CASCADE
PRAGMA foreign_keys = ON;

CREATE TRIGGER fk_user_role_rel_user_id_delete BEFORE DELETE ON t_user_role_rel FOR EACH ROW BEGIN
DELETE FROM t_user_base
WHERE
    id = OLD.user_id;

END;

CREATE TRIGGER fk_user_role_rel_role_id_delete BEFORE DELETE ON t_user_role_rel FOR EACH ROW BEGIN
DELETE FROM t_user_role
WHERE
    id = OLD.role_id;

END;

-- 用户会员等级表
CREATE TABLE IF NOT EXISTS
    t_user_member_level (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 会员等级ID
        "name" TEXT UNIQUE NOT NULL, -- 会员等级名称
        "level" INTEGER UNIQUE NOT NULL, -- 会员等级
        sort INTEGER DEFAULT 0, -- 排序
        "desc" TEXT DEFAULT '', -- 会员描述
        "status" TINYINT NOT NULL DEFAULT 1, -- 状态(0:停用,1:正常)
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE UNIQUE INDEX idx_name ON t_user_member_level ("name");

-- 用户地理位置表
CREATE TABLE IF NOT EXISTS
    t_user_location (
        id INTEGER PRIMARY KEY AUTOINCREMENT, -- 地理位置ID
        user_id INTEGER UNIQUE NOT NULL, -- 用户ID
        province TEXT NOT NULL, -- 省份
        city TEXT NOT NULL, -- 城市
        district TEXT NOT NULL, -- 区/县
        "address" TEXT NOT NULL, -- 详细地址
        postal_code TEXT DEFAULT '', -- 邮政编码
        longitude REAL DEFAULT 0, -- 经度
        latitude REAL DEFAULT 0, -- 纬度
        "desc" TEXT DEFAULT '', -- 描述信息
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
    );

CREATE INDEX idx_user_id ON t_user_location (user_id);
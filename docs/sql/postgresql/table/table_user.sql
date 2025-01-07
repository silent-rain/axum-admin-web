/*用户与身份管理相关表*/
-- 角色表
CREATE TABLE IF NOT EXISTS t_user_role (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL,
    sort INT NULL DEFAULT 0,
    "desc" VARCHAR(200) NULL DEFAULT '',
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (id)
);

COMMENT ON TABLE t_user_role IS '角色表';
COMMENT ON COLUMN t_user_role.id IS '角色ID';
COMMENT ON COLUMN t_user_role.name IS '角色名称';
COMMENT ON COLUMN t_user_role.sort IS '排序';
COMMENT ON COLUMN t_user_role."desc" IS '描述信息';
COMMENT ON COLUMN t_user_role.status IS '状态(0:停用,1:正常)';
COMMENT ON COLUMN t_user_role.created_at IS '创建时间';
COMMENT ON COLUMN t_user_role.updated_at IS '更新时间';

CREATE INDEX idx_name ON t_user_role (name);


-- 用户信息表
CREATE TABLE IF NOT EXISTS t_user_base (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) UNIQUE NOT NULL,
    real_name VARCHAR(32) DEFAULT '',
    gender SMALLINT NOT NULL,
    password VARCHAR(64) NOT NULL,
    status SMALLINT NOT NULL,
    age INT DEFAULT 0,
    date_birth VARCHAR(20) DEFAULT '',
    avatar VARCHAR(200) DEFAULT '',
    intro VARCHAR(200) DEFAULT '',
    "desc" VARCHAR(200) DEFAULT '',
    address VARCHAR(200) DEFAULT '',
    share_code VARCHAR(16) DEFAULT '',
    preferences VARCHAR(200) DEFAULT '',
    department_id INT DEFAULT 0,
    position_id INT DEFAULT 0,
    rank_id INT DEFAULT 0,
    member_level_id INT DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE t_user_base ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

CREATE INDEX idx_username ON t_user_base (username);

COMMENT ON TABLE t_user_base IS '用户信息表';
COMMENT ON COLUMN t_user_base.id IS '用户ID';
COMMENT ON COLUMN t_user_base.username IS '用户名称';
COMMENT ON COLUMN t_user_base.real_name IS '真实姓名';
COMMENT ON COLUMN t_user_base.gender IS '性别(0:男,1:女,2:保密)';
COMMENT ON COLUMN t_user_base.password IS '密码';
COMMENT ON COLUMN t_user_base.status IS '状态(0:停用,1:正常)';
COMMENT ON COLUMN t_user_base.age IS '年龄';
COMMENT ON COLUMN t_user_base.date_birth IS '出生日期';
COMMENT ON COLUMN t_user_base.avatar IS '头像URL';
COMMENT ON COLUMN t_user_base.intro IS '用户个人介绍';
COMMENT ON COLUMN t_user_base."desc" IS '用户描述';
COMMENT ON COLUMN t_user_base.address IS '用户的居住或邮寄地址';
COMMENT ON COLUMN t_user_base.share_code IS '用户分享码';
COMMENT ON COLUMN t_user_base.preferences IS '偏好设置';
COMMENT ON COLUMN t_user_base.department_id IS '所属部门ID';
COMMENT ON COLUMN t_user_base.position_id IS '所属岗位ID';
COMMENT ON COLUMN t_user_base.rank_id IS '所属职级ID';
COMMENT ON COLUMN t_user_base.member_level_id IS '用户会员等级ID';
COMMENT ON COLUMN t_user_base.created_at IS '创建时间';
COMMENT ON COLUMN t_user_base.updated_at IS '更新时间';

-- 用户邮箱表
CREATE TABLE IF NOT EXISTS t_user_email (
    id SERIAL PRIMARY KEY,
    user_id INT UNIQUE NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    "desc" VARCHAR(200) DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

ALTER TABLE t_user_email ALTER COLUMN updated_at SET DEFAULT CURRENT_TIMESTAMP;

CREATE INDEX idx_user_id ON t_user_email (user_id);

ALTER TABLE t_user_email ADD CONSTRAINT fk_user_email_user_id
    FOREIGN KEY (user_id) REFERENCES t_user_base (id) ON DELETE CASCADE ON UPDATE CASCADE;

COMMENT ON TABLE t_user_email IS '用户邮箱';
COMMENT ON COLUMN t_user_email.id IS '邮箱ID';
COMMENT ON COLUMN t_user_email.user_id IS '用户ID';
COMMENT ON COLUMN t_user_email.email IS '邮箱';
COMMENT ON COLUMN t_user_email."desc" IS '描述信息';
COMMENT ON COLUMN t_user_email.created_at IS '创建时间';
COMMENT ON COLUMN t_user_email.updated_at IS '更新时间';

-- 用户手机号表
CREATE TABLE IF NOT EXISTS t_user_phone (
    id SERIAL PRIMARY KEY,
    user_id INT UNIQUE NOT NULL,
    phone VARCHAR(16) UNIQUE NOT NULL,
    "desc" VARCHAR(200) DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_id ON t_user_phone (user_id);

ALTER TABLE t_user_phone ADD CONSTRAINT fk_user_phone_user_id
    FOREIGN KEY (user_id) REFERENCES t_user_base (id) ON DELETE CASCADE ON UPDATE CASCADE;

COMMENT ON TABLE t_user_phone IS '用户手机号';
COMMENT ON COLUMN t_user_phone.id IS '手机号ID';
COMMENT ON COLUMN t_user_phone.user_id IS '用户ID';
COMMENT ON COLUMN t_user_phone.phone IS '手机号码';
COMMENT ON COLUMN t_user_phone."desc" IS '描述信息';
COMMENT ON COLUMN t_user_phone.created_at IS '创建时间';
COMMENT ON COLUMN t_user_phone.updated_at IS '更新时间';


-- 用户区块链钱包表
CREATE TABLE IF NOT EXISTS t_user_blockchain_wallet (
    id SERIAL PRIMARY KEY,
    user_id INT UNIQUE NOT NULL,
    wallet_address VARCHAR(255) UNIQUE NOT NULL,
    mnemonic VARCHAR(255) DEFAULT '',
    private_key VARCHAR(255) DEFAULT '',
    chain_id INT DEFAULT 0,
    "desc" VARCHAR(200) DEFAULT '',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_user_id ON t_user_blockchain_wallet (user_id);
CREATE INDEX idx_wallet_address ON t_user_blockchain_wallet (wallet_address);

ALTER TABLE t_user_blockchain_wallet ADD CONSTRAINT fk_user_blockchain_wallet_user_id
    FOREIGN KEY (user_id) REFERENCES t_user_base (id) ON DELETE CASCADE ON UPDATE CASCADE;

COMMENT ON TABLE t_user_blockchain_wallet IS '用户区块链钱包表';
COMMENT ON COLUMN t_user_blockchain_wallet.id IS '钱包ID';
COMMENT ON COLUMN t_user_blockchain_wallet.user_id IS '用户ID';
COMMENT ON COLUMN t_user_blockchain_wallet.wallet_address IS '钱包地址';
COMMENT ON COLUMN t_user_blockchain_wallet.mnemonic IS '助记词';
COMMENT ON COLUMN t_user_blockchain_wallet.private_key IS '私钥';
COMMENT ON COLUMN t_user_blockchain_wallet.chain_id IS '区块链ID';
COMMENT ON COLUMN t_user_blockchain_wallet."desc" IS '描述信息';
COMMENT ON COLUMN t_user_blockchain_wallet.created_at IS '创建时间';
COMMENT ON COLUMN t_user_blockchain_wallet.updated_at IS '更新时间';

-- 用户角色关系表
CREATE TABLE IF NOT EXISTS t_user_role_rel (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    role_id INT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX uk_user_id_role_id ON t_user_role_rel (user_id, role_id);

ALTER TABLE t_user_role_rel ADD CONSTRAINT fk_user_role_rel_user_id
    FOREIGN KEY (user_id) REFERENCES t_user_base (id) ON DELETE CASCADE ON UPDATE CASCADE;

ALTER TABLE t_user_role_rel ADD CONSTRAINT fk_user_role_rel_role_id
    FOREIGN KEY (role_id) REFERENCES t_user_role (id) ON DELETE CASCADE ON UPDATE CASCADE;

COMMENT ON TABLE t_user_role_rel IS '用户角色关系表';
COMMENT ON COLUMN t_user_role_rel.id IS '自增ID';
COMMENT ON COLUMN t_user_role_rel.user_id IS '用户ID';
COMMENT ON COLUMN t_user_role_rel.role_id IS '角色ID';
COMMENT ON COLUMN t_user_role_rel.created_at IS '创建时间';

-- 用户会员等级表
CREATE TABLE IF NOT EXISTS t_user_member_level (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL,
    level INT UNIQUE NOT NULL,
    sort INT DEFAULT 0,
    "desc" VARCHAR(200) DEFAULT '',
    status SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_name ON t_user_member_level (name);

COMMENT ON TABLE t_user_member_level IS '用户会员等级表';
COMMENT ON COLUMN t_user_member_level.id IS '会员等级ID';
COMMENT ON COLUMN t_user_member_level.name IS '会员等级名称';
COMMENT ON COLUMN t_user_member_level.level IS '会员等级';
COMMENT ON COLUMN t_user_member_level.sort IS '排序';
COMMENT ON COLUMN t_user_member_level.desc IS '会员描述';
COMMENT ON COLUMN t_user_member_level.status IS '状态(0:停用,1:正常)';
COMMENT ON COLUMN t_user_member_level.created_at IS '创建时间';
COMMENT ON COLUMN t_user_member_level.updated_at IS '更新时间';


-- 用户地理位置表
CREATE TABLE IF NOT EXISTS t_user_location (
    id SERIAL PRIMARY KEY, 
    user_id INT UNIQUE NOT NULL, 
    province VARCHAR(50) NOT NULL, 
    city VARCHAR(50) NOT NULL, 
    district VARCHAR(50) NOT NULL, 
    address VARCHAR(255) NOT NULL, 
    postal_code VARCHAR(20) DEFAULT '', 
    longitude DECIMAL(11, 8) DEFAULT 0, 
    latitude DECIMAL(10, 8) DEFAULT 0, 
    "desc" VARCHAR(200) DEFAULT '', 
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP, 
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP 
);

COMMENT ON TABLE t_user_location IS '用户地理位置表';
COMMENT ON COLUMN t_user_location.id IS '地理位置ID';
COMMENT ON COLUMN t_user_location.user_id IS '用户ID';
COMMENT ON COLUMN t_user_location.province IS '省份';
COMMENT ON COLUMN t_user_location.city IS '城市';
COMMENT ON COLUMN t_user_location.district IS '区/县';
COMMENT ON COLUMN t_user_location.address IS '详细地址';
COMMENT ON COLUMN t_user_location.postal_code IS '邮政编码';
COMMENT ON COLUMN t_user_location.longitude IS '经度';
COMMENT ON COLUMN t_user_location.latitude IS '纬度';
COMMENT ON COLUMN t_user_location."desc" IS '描述信息';
COMMENT ON COLUMN t_user_location.created_at IS '创建时间';
COMMENT ON COLUMN t_user_location.updated_at IS '更新时间';

CREATE INDEX idx_user_id ON t_user_location (user_id);
//! 菜单表
//! Entity: [`entity::permission::Menu`]

use sea_orm::{ConnectionTrait, DatabaseBackend, DeriveMigrationName};
use sea_orm_migration::{async_trait, DbErr, MigrationTrait, SchemaManager};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        match manager.get_database_backend() {
            DatabaseBackend::MySql => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS
                    `t_perm_menu` (
                        `id` INT(11) AUTO_INCREMENT NOT NULL COMMENT '菜单ID',
                        `pid` INT(20) NULL DEFAULT 0 COMMENT '父菜单ID',
                        `title` VARCHAR(20) NOT NULL COMMENT '菜单名称',
                        `icon_class` VARCHAR(20) NULL DEFAULT '' COMMENT 'Icon图标类',
                        `menu_type` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '菜单类型(0:菜单,1:按钮)',
                        `open_method` TINYINT(1) NOT NULL DEFAULT 0 COMMENT '打开方式(0:组件,1:内链,2:外链)',
                        `path` VARCHAR(255) NULL DEFAULT '' COMMENT '路由地址',
                        `component_path` VARCHAR(255) NULL DEFAULT '' COMMENT '组件路径',
                        `redirect_to` VARCHAR(255) NULL DEFAULT '' COMMENT '路由重定向',
                        `link` VARCHAR(255) NULL DEFAULT '' COMMENT '链接地址:站内链地址/站外链地址',
                        `link_target` VARCHAR(20) NULL DEFAULT '_blank' COMMENT '链接跳转方式,_blank/_self',
                        `is_hidden` TINYINT(1) NULL DEFAULT 1 COMMENT '是否隐藏(0:显示,1:隐藏)',
                        `is_always_show_root` TINYINT(1) NULL DEFAULT 1 COMMENT '是否始终显示根菜单(0:隐藏,1:显示)',
                        `permission` VARCHAR(200) NULL DEFAULT '' COMMENT '权限标识',
                        `sort` INT(11) NULL DEFAULT 0 COMMENT '排序',
                        `desc` VARCHAR(200) NULL DEFAULT '' COMMENT '描述信息',
                        `status` TINYINT(1) NOT NULL DEFAULT 1 COMMENT '状态(0:停用,1:正常)',
                        `created_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
                        `updated_at` DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP COMMENT '更新时间',
                        PRIMARY KEY (`id`)
                    ) ENGINE = InnoDB DEFAULT CHARACTER SET = utf8mb4 COMMENT '菜单表';

                    CREATE INDEX idx_pid ON t_perm_menu (`pid`);
                    CREATE INDEX idx_title ON t_perm_menu (`title`);
                    ",
                )
                .await?;
            }
            DatabaseBackend::Postgres => {
                db.execute_unprepared(
                    r#"
                    CREATE TABLE IF NOT EXISTS "t_perm_menu" (
                        "id" SERIAL PRIMARY KEY,
                        "pid" INTEGER DEFAULT 0,
                        "title" VARCHAR(20) NOT NULL,
                        "icon_class" VARCHAR(20) DEFAULT '',
                        "menu_type" SMALLINT NOT NULL DEFAULT 0,
                        "open_method" SMALLINT NOT NULL DEFAULT 0,
                        "path" VARCHAR(255) DEFAULT '',
                        "component_path" VARCHAR(255) DEFAULT '',
                        "redirect_to" VARCHAR(255) DEFAULT '',
                        "link" VARCHAR(255) DEFAULT '',
                        "link_target" VARCHAR(20) DEFAULT '_blank',
                        "is_hidden" SMALLINT DEFAULT 1,
                        "is_always_show_root" SMALLINT DEFAULT 1,
                        "permission" VARCHAR(200) DEFAULT '',
                        "sort" INTEGER DEFAULT 0,
                        "desc" VARCHAR(200) DEFAULT '',
                        "status" SMALLINT NOT NULL DEFAULT 1,
                        "created_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        "updated_at" TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
                    );

                    CREATE INDEX idx_t_perm_menu_pid ON t_perm_menu ("pid");
                    CREATE INDEX idx_t_perm_menu_title ON t_perm_menu ("title");

                    COMMENT ON TABLE t_perm_menu IS '菜单表';
                    COMMENT ON COLUMN t_perm_menu.id IS '菜单ID';
                    COMMENT ON COLUMN t_perm_menu.pid IS '父菜单ID';
                    COMMENT ON COLUMN t_perm_menu.title IS '菜单名称';
                    COMMENT ON COLUMN t_perm_menu.icon_class IS 'Icon图标类';
                    COMMENT ON COLUMN t_perm_menu.menu_type IS '菜单类型(0:菜单,1:按钮)';
                    COMMENT ON COLUMN t_perm_menu.open_method IS '打开方式(0:组件,1:内链,2:外链)';
                    COMMENT ON COLUMN t_perm_menu.path IS '路由地址';
                    COMMENT ON COLUMN t_perm_menu.component_path IS '组件路径';
                    COMMENT ON COLUMN t_perm_menu.redirect_to IS '路由重定向';
                    COMMENT ON COLUMN t_perm_menu.link IS '链接地址:站内链地址/站外链地址';
                    COMMENT ON COLUMN t_perm_menu.link_target IS '链接跳转方式,_blank/_self';
                    COMMENT ON COLUMN t_perm_menu.is_hidden IS '是否隐藏(0:显示,1:隐藏)';
                    COMMENT ON COLUMN t_perm_menu.is_always_show_root IS '是否始终显示根菜单(0:隐藏,1:显示)';
                    COMMENT ON COLUMN t_perm_menu.permission IS '权限标识';
                    COMMENT ON COLUMN t_perm_menu.sort IS '排序';
                    COMMENT ON COLUMN t_perm_menu.desc IS '描述信息';
                    COMMENT ON COLUMN t_perm_menu.status IS '状态(0:停用,1:正常)';
                    COMMENT ON COLUMN t_perm_menu.created_at IS '创建时间';
                    COMMENT ON COLUMN t_perm_menu.updated_at IS '更新时间';
                    "#,
                )
                .await?;
            }
            DatabaseBackend::Sqlite => {
                db.execute_unprepared(
                    "
                    CREATE TABLE IF NOT EXISTS `t_perm_menu` ( -- 菜单表
                        `id` INTEGER PRIMARY KEY AUTOINCREMENT, -- 菜单ID
                        `pid` INTEGER DEFAULT 0, -- 父菜单ID
                        `title` VARCHAR(20) NOT NULL, -- 菜单名称
                        `icon_class` VARCHAR(20) DEFAULT '', -- Icon图标类
                        `menu_type` TINYINT NOT NULL DEFAULT 0, -- 菜单类型(0:菜单,1:按钮)
                        `open_method` TINYINT NOT NULL DEFAULT 0, -- 打开方式(0:组件,1:内链,2:外链)
                        `path` VARCHAR(255) DEFAULT '', -- 路由地址
                        `component_path` VARCHAR(255) DEFAULT '', -- 组件路径
                        `redirect_to` VARCHAR(255) DEFAULT '', -- 路由重定向
                        `link` VARCHAR(255) DEFAULT '', -- 链接地址:站内链地址/站外链地址
                        `link_target` VARCHAR(20) DEFAULT '_blank', -- 链接跳转方式,_blank/_self
                        `is_hidden` TINYINT DEFAULT 1, -- 是否隐藏(0:显示,1:隐藏)
                        `is_always_show_root` TINYINT DEFAULT 1, -- 是否始终显示根菜单(0:隐藏,1:显示)
                        `permission` VARCHAR(200) DEFAULT '', -- 权限标识
                        `sort` INTEGER DEFAULT 0, -- 排序
                        `desc` VARCHAR(200) DEFAULT '', -- 描述信息
                        `status` TINYINT NOT NULL DEFAULT 1, -- 状态(0:停用,1:正常)
                        `created_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 创建时间
                        `updated_at` TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新时间
                    );

                    CREATE INDEX idx_t_perm_menu_pid ON t_perm_menu (`pid`);
                    CREATE INDEX idx_t_perm_menu_title ON t_perm_menu (`title`);
                    ",
                )
                .await?;
            }
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE `t_perm_menu`")
            .await?;

        Ok(())
    }
}

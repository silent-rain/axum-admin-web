//! Mock 模拟测试

use std::sync::Arc;

use sea_orm::{ConnectionTrait, DbErr, EntityTrait, Schema, Statement};
use sea_orm_migration::{MigrationTrait, SchemaManager};

use crate::{Options, Pool, PoolTrait, config::Level};

pub struct Mock {
    pool: Arc<dyn PoolTrait>,
}

impl Mock {
    /// 从迁移文件创建表
    pub async fn migration_migrations(
        self,
        migrations: Vec<&dyn MigrationTrait>,
    ) -> Result<Self, DbErr> {
        for migration in migrations {
            let manager = SchemaManager::new(self.pool.db());
            migration.up(&manager).await?;
        }

        Ok(self)
    }

    /// 从实体创建表
    ///
    pub async fn migration_entity<E: EntityTrait>(self, entity: E) -> Result<Self, DbErr> {
        let builder = self.pool.db().get_database_backend();
        let schema = Schema::new(builder);
        self.pool
            .db()
            .execute(builder.build(&schema.create_table_from_entity(entity)))
            .await?;

        Ok(self)
    }

    // 从实体创建表
    // 失败的示例
    // pub async fn migration_entities<E, I>(self, entities: I) -> Result<Self, DbErr>
    // where
    //     E: EntityTrait + Sized,
    //     I: IntoIterator<Item = E>,
    // {
    //     let builder = self.pool.db().get_database_backend();
    //     let schema = Schema::new(builder);
    //     for entity in entities {
    //         self.pool
    //             .db()
    //             .execute(builder.build(&schema.create_table_from_entity(entity)))
    //             .await?;
    //     }

    //     Ok(self)
    // }

    /// 从sql字符串创建表
    pub async fn migration_str(self, sql: &str) -> Result<Self, DbErr> {
        let stmt = Statement::from_sql_and_values(self.pool.db().get_database_backend(), sql, []);
        self.pool.db().execute(stmt).await?;

        Ok(self)
    }

    /// 连接数据库
    async fn connect() -> Result<Arc<dyn PoolTrait>, DbErr> {
        // Connecting SQLite
        let db_url = "sqlite::memory:".to_string();
        let opt = Options {
            logging_enable: true,
            logging_level: Level::Info,
            ..Default::default()
        };
        let db = Pool::connect(db_url, opt).await?;
        let pool = Pool::form_connect(db);

        Ok(Arc::new(pool))
    }

    /// 构建者
    pub async fn builder() -> Result<Self, DbErr> {
        let pool = Self::connect().await?;
        Ok(Mock { pool })
    }

    pub fn build(self) -> Arc<dyn PoolTrait> {
        self.pool
    }
}

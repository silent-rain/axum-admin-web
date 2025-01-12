//! db session sore
use std::sync::Arc;

use async_trait::async_trait;
use chrono::Local;
use database::PoolTrait;
use sea_orm::Set;
use time::OffsetDateTime;
use tower_sessions::{
    ExpiredDeletion, SessionStore,
    session::{Id, Record},
    session_store::{self, Error},
};
use tracing::error;

use crate::dao::UserSessionDao;
use entity::user::user_session;

/// A session store that lives only in memory.
///
/// This is useful for testing but not recommended for real applications.
///
/// # Examples
///
/// ```rust
/// use tower_sessions::DbStore;
/// DbStore::new();
/// ```
pub struct DbStore {
    user_session_dao: UserSessionDao,
}

impl std::fmt::Debug for DbStore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 使用 write! 宏来格式化输出
        write!(f, "db store")
    }
}

impl DbStore {
    pub fn new(db: Arc<dyn PoolTrait>) -> Self {
        DbStore {
            user_session_dao: UserSessionDao::new(db),
        }
    }
}

#[async_trait]
impl ExpiredDeletion for DbStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        self.user_session_dao
            .delete_expired()
            .await
            .map_err(|err| {
                error!("delete expired session failed, err: {err}");
                Error::Backend("delete expired session failed".to_string())
            })?;
        Ok(())
    }
}

#[async_trait]
impl SessionStore for DbStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let data = self
            .user_session_dao
            .info(record.id.to_string())
            .await
            .map_err(|err| {
                error!("read session failed, err: {err}");
                Error::Backend("read session failed".to_string())
            })?;

        if data.is_some() {
            // Session ID collision mitigation.
            record.id = Id::default();
        }

        let active_model = user_session::ActiveModel {
            session_id: Set(record.id.to_string()),
            // expiry_date: Set(record.expiry_date),
            status: Set(true),
            created_at: Set(Local::now().naive_local()),
            ..Default::default()
        };

        self.user_session_dao
            .create(active_model)
            .await
            .map_err(|err| {
                error!("create session failed, err: {err}");
                Error::Backend("create session failed".to_string())
            })?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let session_id = record.id.to_string();
        let active_model = user_session::ActiveModel {
            session_id: Set(session_id),
            // expiry_date: Set(record.expiry_date.unix_timestamp()),
            ..Default::default()
        };

        // self.user_session_dao
        //     .update(session_id, active_model)
        //     .await
        //     .map_err(|err| {
        //         error!("update session failed, err: {err}");
        //         Error::Backend("update session failed".to_string())
        //     })?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let data = self
            .user_session_dao
            .info(session_id.to_string())
            .await
            .map_err(|err| {
                error!("read session failed, err: {err}");
                Error::Backend("read session failed".to_string())
            })?;

        let data = match data {
            Some(v) => v,
            None => {
                error!("invalid session, not found");
                return Ok(None);
            }
        };

        if !data.status {
            error!("invalid session, already expired");
            return Ok(None);
        }

        // 再次判断是否过期
        // if !is_active(data.expiry_date) {
        //     error!("invalid session, already expired");

        //     // 删除过期会话
        //     self.delete(&session_id).await?;

        //     return Err(Error::Backend(
        //         "invalid session, already expired".to_string(),
        //     ));
        // }

        // Ok(Some(Record {
        //     id: data.session_id,
        //     expiry_date: data.expiry_date,
        //     // data:data.id,
        // }))

        Ok(None)
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let _ = self
            .user_session_dao
            .delete(session_id.to_string())
            .await
            .map_err(|err| {
                error!("delete session failed, err: {err}");
                Error::Backend("delete session failed".to_string())
            })?;
        Ok(())
    }
}

fn is_active(expiry_date: OffsetDateTime) -> bool {
    expiry_date > OffsetDateTime::now_utc()
}

#[cfg(test)]
mod tests {
    use time::Duration;

    use database::{Options, Pool};

    use super::*;

    #[tokio::test]
    async fn test_create() -> anyhow::Result<()> {
        let db_url = "sqlite::memory:".to_owned();
        let pool = Pool::new(db_url, Options::default()).await?;
        let store = DbStore::new(Arc::new(pool));

        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        println!("record: {:#?}", record);
        assert!(store.create(&mut record).await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_save() -> anyhow::Result<()> {
        let db_url = "sqlite::memory:".to_owned();
        let pool = Pool::new(db_url, Options::default()).await?;
        let store = DbStore::new(Arc::new(pool));

        let record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        assert!(store.save(&record).await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_load() -> anyhow::Result<()> {
        let db_url = "sqlite::memory:".to_owned();
        let pool = Pool::new(db_url, Options::default()).await?;
        let store = DbStore::new(Arc::new(pool));

        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        let loaded_record = store.load(&record.id).await.unwrap();
        assert_eq!(Some(record), loaded_record);

        Ok(())
    }

    #[tokio::test]
    async fn test_delete() -> anyhow::Result<()> {
        let db_url = "sqlite::memory:".to_owned();
        let pool = Pool::new(db_url, Options::default()).await?;
        let store = DbStore::new(Arc::new(pool));

        let mut record = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date: OffsetDateTime::now_utc() + Duration::minutes(30),
        };
        store.create(&mut record).await.unwrap();
        assert!(store.delete(&record.id).await.is_ok());
        assert_eq!(None, store.load(&record.id).await.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn test_create_id_collision() -> anyhow::Result<()> {
        let db_url = "sqlite::memory:".to_owned();
        let pool = Pool::new(db_url, Options::default()).await?;
        let store = DbStore::new(Arc::new(pool));

        let expiry_date = OffsetDateTime::now_utc() + Duration::minutes(30);
        let mut record1 = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date,
        };
        let mut record2 = Record {
            id: Default::default(),
            data: Default::default(),
            expiry_date,
        };
        store.create(&mut record1).await.unwrap();
        record2.id = record1.id; // Set the same ID for record2
        store.create(&mut record2).await.unwrap();
        assert_ne!(record1.id, record2.id); // IDs should be different

        Ok(())
    }
}

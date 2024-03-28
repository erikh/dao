use super::*;
use crate::bind;
use anyhow::{anyhow, Result};
use sqlx::prelude::*;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct SqliteDB {
    connection: Arc<Mutex<SqlitePool>>,
}

impl SqliteDB {
    pub async fn new(database: &str) -> Result<Self> {
        Ok(Self {
            connection: Arc::new(Mutex::new(
                SqlitePool::connect(database)
                    .await
                    .map_err(|e| anyhow!(e))?,
            )),
        })
    }

    pub async fn create<'a, T>(&self, obj: impl QueryGenerator<'a, T, Sqlite>) -> Result<i64>
    where
        T: Type<Sqlite> + Encode<'a, Sqlite> + Send + 'a,
    {
        let mut tx = self.connection.lock().await.begin().await?;
        let statement = obj.create(QueryType::Sqlite);
        let query = sqlx::query(&statement);

        let res = tx.fetch_one(bind!(obj, query, vec!["name"])).await?.get(0);

        tx.commit().await?;
        Ok(res)
    }

    pub async fn delete<'a, T>(&self, obj: impl QueryGenerator<'a, T, Sqlite>) -> Result<()>
    where
        T: Type<Sqlite> + Encode<'a, Sqlite> + Send,
    {
        let mut tx = self.connection.lock().await.begin().await?;

        tx.execute(bind!(sqlx::query(&obj.delete(QueryType::Sqlite)), obj.id()))
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn update<'a, T>(&self, obj: impl QueryGenerator<'a, T, Sqlite>) -> Result<()>
    where
        T: Type<Sqlite> + Encode<'a, Sqlite> + Send,
    {
        let mut tx = self.connection.lock().await.begin().await?;

        tx.execute(sqlx::raw_sql(&obj.update(QueryType::Sqlite)))
            .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn exists<'a, T>(&self, obj: impl QueryGenerator<'a, T, Sqlite>) -> Result<bool>
    where
        T: Type<Sqlite> + Encode<'a, Sqlite> + Send,
    {
        let mut tx = self.connection.lock().await.begin().await?;

        let res = tx
            .fetch_one(sqlx::raw_sql(&obj.exists(QueryType::Sqlite)))
            .await
            .is_ok();

        tx.commit().await?;

        Ok(res)
    }

    pub async fn count<'a, T>(&self, obj: impl QueryGenerator<'a, T, Sqlite>) -> Result<i64>
    where
        T: Type<Sqlite> + Encode<'a, Sqlite> + Send,
    {
        let mut tx = self.connection.lock().await.begin().await?;

        let res = tx
            .fetch_one(sqlx::raw_sql(&obj.count(QueryType::Sqlite)))
            .await?
            .get(0);

        tx.commit().await?;

        Ok(res)
    }
}

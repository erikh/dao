pub mod sqlite;
pub mod types;

use anyhow::Result;
use sqlx::{Encode, Sqlite, Type};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryType {
    Sqlite,
}

#[macro_export]
macro_rules! bind {
    ($obj:expr, $query:expr, $binds:expr) => {{
        let mut query = $query;
        for item in $($binds)* {
            query = query.bind($obj.value(item)?)
        }

        query
    }};
}

pub trait QueryGenerator<'a, T, DB>
where
    DB: sqlx::Database,
    T: Type<DB> + Encode<'a, DB> + Send,
{
    fn id(&self) -> Option<i64>;
    fn value(&self, column: &str) -> Result<T>;
    fn bind_columns(&self) -> Vec<String>;
    fn create(&self, typ: QueryType) -> &'a str;
    fn delete(&self, typ: QueryType) -> &'a str;
    fn update(&self, typ: QueryType) -> &'a str;
    fn exists(&self, typ: QueryType) -> &'a str;
    fn count(&self, typ: QueryType) -> &'a str;
}

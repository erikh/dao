pub mod sqlite;
pub mod types;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryType {
    Sqlite,
}

#[macro_export]
macro_rules! bind {
    ($query:expr, $($binds:expr),*) => {{
        let mut query = $query;
        for item in $($binds)* {
            query = $query.bind(item)
        }

        query
    }};
    ($query:item, $binds:expr) => {{
        let mut query = $query;
        for item in $binds* {
            query = $query.bind(item)
        }

        query
    }};
}

pub trait QueryGenerator<'a> {
    fn id(&self) -> Option<i64>;
    fn create(&self, typ: QueryType) -> &'a str;
    fn delete(&self, typ: QueryType) -> &'a str;
    fn update(&self, typ: QueryType) -> &'a str;
    fn exists(&self, typ: QueryType) -> &'a str;
    fn count(&self, typ: QueryType) -> &'a str;
}

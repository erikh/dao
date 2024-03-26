pub mod sqlite;
pub mod types;

use sqlx::{database::Database, query::Query, Arguments};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryType {
    Sqlite,
}

pub trait QueryGenerator<'a, DB, A>
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64>;

    fn bind_id(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A> {
        query.bind(self.id())
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>;

    fn create(&self, typ: QueryType) -> &'a str;
    fn delete(&self, typ: QueryType) -> &'a str;
    fn update(&self, typ: QueryType) -> &'a str;
    fn exists(&self, typ: QueryType) -> &'a str;
    fn count(&self, typ: QueryType) -> &'a str;
}

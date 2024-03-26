use super::*;
use sqlx::query::Query;

#[derive(Debug, Clone)]
pub struct Node {
    id: Option<i64>,
    name: String,
    key: String,
    address: String,
    username: String,
    federating: bool,
    alive: bool,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for Node
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
            .bind(self.name)
            .bind(self.key)
            .bind(self.address)
            .bind(self.username)
            .bind(self.federating)
            .bind(self.alive)
    }

    fn count(&self, _typ: QueryType) -> &'a str {
        "select count(*) from nodes"
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        "insert into nodes (name, key, address, username, federating, alive) values (?, ?, ?, ?, ?, ?) returning id"
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        "delete from nodes where id=?"
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        "update nodes set name=?, key=?, address=?, username=?, federating=?, alive=?) where id=?"
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        "select 1 from nodes where id=?"
    }
}

#[derive(Debug, Clone)]
pub struct Plan {
    id: Option<i64>,
    node: Node,
    failures: u32,
    scheduled: bool,
    last_deployed: chrono::DateTime<chrono::Local>,
    plan_node: PlanNode,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for Plan
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
    }
    fn count(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct PlanNode {
    id: Option<i64>,
    node: Node,
    schedule: Schedule,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for PlanNode
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
    }

    fn count(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct Schedule {
    id: Option<i64>,
    manifest: crate::manifest::Manifest,
    count: u64,
    user: User,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for Schedule
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
    }

    fn count(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: Option<i64>,
    username: String,
    key: String,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for User
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
    }

    fn count(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct Status {
    id: Option<i64>,
    node: Node,
    cpu: u64,
    mem: u64,
    storage: u64,
    last_queried: chrono::DateTime<chrono::Local>,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for Status
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
    }

    fn count(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    id: Option<i64>,
    user: User,
    time: chrono::DateTime<chrono::Local>,
    action: String,
}

impl<'a, DB, A> QueryGenerator<'a, DB, A> for Log
where
    DB: Database,
    A: Arguments<'a>,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn bind(&self, query: Query<'a, DB, A>) -> Query<'a, DB, A>
    where
        DB: Database,
    {
        query
    }

    fn count(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn create(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn delete(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn update(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }

    fn exists(&self, _typ: QueryType) -> &'a str {
        Default::default()
    }
}

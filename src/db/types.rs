use super::*;

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

impl<'a, DB> QueryGenerator<'a, DB> for Node
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

impl<'a, DB> QueryGenerator<'a, DB> for Plan
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

impl<'a, DB> QueryGenerator<'a, DB> for PlanNode
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

impl<'a, DB> QueryGenerator<'a, DB> for Schedule
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

impl<'a, DB> QueryGenerator<'a, DB> for User
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

impl<'a, DB> QueryGenerator<'a, DB> for Status
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

impl<'a, DB> QueryGenerator<'a, DB> for Log
where
    DB: sqlx::Database,
{
    fn id(&self) -> Option<i64> {
        self.id
    }

    fn binds<T>(&self) -> Vec<T>
    where
        T: 'a + Send + Encode<'a, DB> + Type<DB>,
    {
        vec![]
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

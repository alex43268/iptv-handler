pub mod models;
pub mod services;
use models::{Attribute, ExtInf, M3u, Provider};
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{migrate, Error, MySql, MySqlConnection, Pool};
use std::fmt::Debug;
use std::str::FromStr;
use std::sync::Arc;

pub type ConnectionPool = Pool<MySql>;
pub type Connection = MySqlConnection;

pub async fn connect(database_url: String) -> ConnectionPool {
    let connection_options =
        MySqlConnectOptions::from_str(&database_url).expect("Parsing of connection string failed");

    MySqlPoolOptions::new()
        .connect_with(connection_options)
        .await
        .expect("creating database connection")
}

pub async fn handle_migrations(pool: &ConnectionPool) {
    migrate!("./migrations")
        .run(pool)
        .await
        .expect("running database migrations");
}

#[async_trait::async_trait]
pub trait CRUD<TReturn, TInsert>: Send + Sync + Debug {
    async fn get(&self, tx: &mut Connection, id: u64) -> Result<TReturn, Error>;
    async fn insert(&self, tx: &mut Connection, model: TInsert) -> Result<u64, Error>;
    async fn delete(&self, tx: &mut Connection, id: u64) -> Result<u64, Error>;
}

#[derive(Debug)]
pub struct DB {
    pub pool: Arc<ConnectionPool>,

    pub provider: Provider,
    pub m3u: M3u,
    pub extinf: ExtInf,
    pub attribute: Attribute,
}

pub async fn init_db(pool: ConnectionPool) -> DB {
    DB {
        pool: Arc::new(pool.clone()),

        provider: Provider {},
        attribute: Attribute {},
        m3u: M3u {},
        extinf: ExtInf {},
    }
}

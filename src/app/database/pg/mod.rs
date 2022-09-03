#![allow(dead_code)]

use actix_web::web;
use diesel::{
    PgConnection,
    r2d2::{Builder, ConnectionManager, Pool}
};
use scheduled_thread_pool::ScheduledThreadPool;
use std::{
    error::Error,
    fmt,
    ops::DerefMut,
    sync::Arc,
    time::Duration
};

/// Exposes an adapter that interfaces with utilities that exist postgres-side.
pub mod funcs;

/// Number of connections per Actix worker.
pub const CONNS_PER_WORKER: u8 = 10;

/// Number of threads for the connection manager to handle async operations.
pub const THREAD_POOL_SIZE: u8 = 3;

#[cfg(test)]
const PG_URI: &'static str = "PG_TEST_URI";

#[cfg(not(test))]
const PG_URI: &'static str = "PG_URI";

/// A facade over a connection pool object which is meant to be used on a per worker basis.
/// One instance of `PgConnPool` per worker essentially.
pub struct PgConnPool {
    uri: String,
    thread_pool_size: u8,
    min_idle: u32,
    idle_timeout: Duration,
    max_lifetime: Duration,
    connection_timeout: Duration,
    connection_pool_size: u8,
    pool: Pool<Manager>
}

/// App's connection manager for Postgres-type connections.
pub type Manager = ConnectionManager<PgConnection>;

impl PgConnPool {
    /// Initializes `PgConnPool`. Meant to be used on a per worker basis.
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let uri = dotenv::var(PG_URI)?;

        let connection_pool_size = CONNS_PER_WORKER;
        let thread_pool = ScheduledThreadPool::new(THREAD_POOL_SIZE.into());

        let idle_timeout = Duration::from_secs(10 * 60);
        let max_lifetime = Duration::from_secs(30 * 60);
        let connection_timeout = Duration::from_secs(5);
        let min_idle = 2;

        let connection_manager: Manager = ConnectionManager::new(uri.clone());

        let builder: Builder<Manager> = Builder::new()
            .max_size(connection_pool_size.into())
            .thread_pool(Arc::new(thread_pool))
            .idle_timeout(Some(idle_timeout))
            .max_lifetime(Some(max_lifetime))
            .connection_timeout(connection_timeout)
            .min_idle(Some(min_idle));

        let pool = builder.build(connection_manager)?;

        Ok(Self {
            uri,
            min_idle,
            idle_timeout,
            connection_pool_size,
            connection_timeout,
            max_lifetime,
            pool,
            thread_pool_size: THREAD_POOL_SIZE
        })
    }

    /// Takes in a closure that receives a borrowed pooled connection to postgres, allowing database
    /// operations to occur asynchronously within the scope of said closure. Example:
    ///
    /// ```rust
    /// let pg = PgConnPool::init().unwrap();
    ///
    /// let some_data = pg.with_conn::<_, SomeType>(|pg_conn| {
    ///     // Do something with the connection here that returns that ultimately
    ///     // returns `Result<SomeType, <Box dyn Error>>.
    /// }).await?;
    /// ```
    ///
    /// Diesel operations on blocking by nature, so fetching a pooled connection and `op` are sent
    /// to a pooled blocking thread where blocking is acceptable.
    pub async fn with_conn<F, U>(&self, op: F) -> Result<U, Box<dyn Error + Send + Sync>>
    where
        F: FnOnce(&mut PgConnection) -> Result<U, Box<dyn Error + Send + Sync>> + Send + 'static,
        U: Send + 'static
    {
        let cloned_pool = self.pool.clone();

        web::block(move || -> Result<U, Box<dyn Error + Send + Sync>> {
            let mut pooled_conn = cloned_pool.get()?;
            
            let raw_conn = pooled_conn.deref_mut();

            Ok(op(raw_conn)?)
        }).await?
    }
}

impl fmt::Display for PgConnPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "uri={} min_idle={} idle_timeout={:?} connection_pool_size={} connection_timeout={:?} max_lifetime={:?}",
            self.uri,
            self.min_idle,
            self.idle_timeout,
            self.connection_pool_size,
            self.connection_timeout,
            self.max_lifetime,
        )
    }
}

#![allow(dead_code)]

use diesel::{
    Connection,
    PgConnection,
    r2d2::{Builder, ConnectionManager, PooledConnection, Pool}
};
use scheduled_thread_pool::ScheduledThreadPool;
use std::{
    error::Error,
    fmt,
    ops::Deref,
    sync::Arc,
    time::Duration
};

/// Number of connections per Actix worker.
pub const CONNS_PER_WORKER: u8 = 5;

/// Number of threads for the connection manager to handle async operations.
pub const THREAD_POOL_SIZE: u8 = 3;

#[cfg(test)]
const PG_URI: &'static str = "PG_TEST_URI";

#[cfg(not(test))]
const PG_URI: &'static str = "PG_URI";

/// A facade over a connection pool object which is meant to be used on a per worker basis.
/// One instance of `Pg` per worker essentially.
pub struct Pg {
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

/// A connection from the pool.  
pub type PgConn = PooledConnection<Manager>;

impl Pg {
    /// Initializes `Pg`. Meant to be used on a per worker basis.
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

    fn execute(&self, sql: &str) -> Result<usize, String> {
        let pooled_conn = match self.pool.get() {
            Ok(conn) => conn,
            Err(e) => return Err(e.to_string())
        };

        let pg_conn = &*pooled_conn;

        match pg_conn.execute(sql) {
            Ok(val) => Ok(val),
            Err(e) => return Err(e.to_string())
        }
    }
}

impl fmt::Display for Pg {
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

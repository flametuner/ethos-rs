use std::env;

use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

use crate::errors::StoreError;

pub fn create_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(database_url);
    Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool.")
}
pub struct ConnectionPool {
    pool: Pool<ConnectionManager<PgConnection>>,
}
impl ConnectionPool {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        ConnectionPool { pool }
    }

    pub fn get(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, StoreError> {
        Ok(self.pool.get()?)
    }
}

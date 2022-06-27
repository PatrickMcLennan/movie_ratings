use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::pg::PgConnection;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_postgres_connection() -> Result<PgPool, PoolError> {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

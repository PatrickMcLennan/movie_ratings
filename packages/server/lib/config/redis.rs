use std::env;
use redis::{Connection, RedisResult};

pub fn connect_to_redis() -> RedisResult<Connection> {
	let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url)?;
    let con = client.get_connection()?;
    Ok(con)
}

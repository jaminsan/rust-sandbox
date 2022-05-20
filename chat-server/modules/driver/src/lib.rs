use bb8_redis::bb8::Pool;
use bb8_redis::RedisConnectionManager;
use futures::executor::block_on;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

pub mod comment_driver;
mod test_rs_streams;

static REDIS_CONNECTION_POOL: Lazy<RwLock<Pool<RedisConnectionManager>>> = Lazy::new(|| {
    let manager = RedisConnectionManager::new("redis://localhost:7000").unwrap();
    let pool =
        block_on(
            bb8_redis::bb8::Pool::builder()
                .max_size(10)
                .build(manager)
        ).unwrap();

    RwLock::new(pool)
});

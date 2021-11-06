use bb8_redis::{redis, RedisConnectionManager};
use bb8_redis::bb8::Pool;
use bb8_redis::redis::{
    AsyncCommands,
    FromRedisValue,
    RedisResult,
};
use futures::{Stream, StreamExt};

use crate::REDIS_CONNECTION_POOL;

pub async fn push(comment: CommentValue) {
    let pool = get_pool().await;
    let mut con = pool.get().await.unwrap();
    let key = channel(&comment.room_id);
    let value = serde_json::to_string(&comment).unwrap();

    let _: () = redis::cmd("LPUSH")
        .arg(&[key, value])
        .query_async(&mut *con)
        .await
        .unwrap();
}

pub async fn publish(comment: CommentValue) {
    let pool = get_pool().await;
    let mut con = pool.get().await.unwrap();

    let value = serde_json::to_string(&comment).unwrap();

    let _: () =
        con
            .publish(
                channel(&comment.room_id),
                value,
            ).await.unwrap();
}

pub async fn subscribe(room_id: &String) -> impl Stream<Item=CommentValue> {
    let mut pubsub = get_new_connection().await.into_pubsub();

    let _: () =
        pubsub
            .subscribe(channel(room_id))
            .await
            .unwrap();

    pubsub
        .into_on_message()
        .map(|message| {
            let t: String = message.get_payload().unwrap();
            serde_json::from_str::<CommentValue>(t.as_str()).unwrap()
        })
}

async fn get_new_connection() -> redis::aio::Connection {
    REDIS_CONNECTION_POOL
        .read()
        .await
        .dedicated_connection()
        .await
        .unwrap()
}

async fn get_pool<'p>() -> Pool<RedisConnectionManager> {
    REDIS_CONNECTION_POOL.read().await.clone()
}

fn channel(room_id: &String) -> String {
    format!("comments:{}", room_id)
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CommentValue {
    pub comment_id: String,
    pub room_id: String,
    pub audience_id: String,
    pub text: String,
}

impl FromRedisValue for CommentValue {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let s: String = redis::from_redis_value(v)?;
        Ok(serde_json::from_str(s.as_str()).unwrap())
    }
}

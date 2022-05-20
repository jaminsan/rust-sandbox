use redis::{FromRedisValue, RedisResult};
use redis::streams::StreamReadReply;

#[cfg(test)]
mod test {
    use bb8_redis::redis;
    use testcontainers::{clients, images};

    use crate::test_rs_streams::{TestData, xadd, xread_block};

    static TEST_STREAM: &str = "test-stream";
    static TEST_STREAM_ENTRY: &str = "test_data";
    static N_REPEAT: i32 = 10000000;

    #[tokio::test]
    async fn test_xread() {
        let docker = clients::Cli::default();
        // let redis_image = images::generic::GenericImage::new("redis", "latest");
        let redis_port = 55004;
        // let redis_image = images::redis::Redis::default();
        // let redis_port = docker.run(redis_image).get_host_port(6379);
        println!("{}", redis_port);
        let client = redis::Client::open(format!("redis://127.0.0.1:{}/", redis_port)).unwrap();

        // start listening redis stream
        let _client = client.clone();
        let read_stream_fut = tokio::spawn(async move {
            let mut con = _client.get_async_connection().await.unwrap();
            let mut current_read_id = "0".to_string();
            loop {
                for (id, value) in xread_block::<TestData>(&mut con, TEST_STREAM, &current_read_id, TEST_STREAM_ENTRY, ).await {
                    println!("id:{} value_of_field_{}:{:?}", id, TEST_STREAM_ENTRY, value);
                    current_read_id = id;
                    if value.v == N_REPEAT {
                        return ();
                    }
                }
            }
        });

        // add entry to stream
        let mut con = client.get_async_connection().await.unwrap();
        for n in 1..(N_REPEAT + 1) {
            let v = serde_json::to_string(&TestData { v: n }).unwrap();
            xadd(&mut con, TEST_STREAM, (TEST_STREAM_ENTRY, &v)).await;
        }

        read_stream_fut.await.unwrap();
    }
}

async fn xadd(con: &mut redis::aio::Connection, stream_key: &str, entry: (&str, &str)) {
    let _: () = redis::cmd("XADD")
        .arg(stream_key)
        .arg("*")
        .arg(&[
            (entry.0, entry.1)
        ]).query_async(con).await.unwrap();
}

async fn xread_block<R: FromRedisValue>(
    con: &mut redis::aio::Connection,
    stream_key: &str,
    entry_id_read_after: &str,
    entry_field: &str,
) -> Vec<(String, R)> {
    let srr: StreamReadReply =
        redis::cmd("XREAD")
            .arg("BLOCK").arg("0") // block without timeout
            .arg("STREAMS")
            .arg(stream_key)
            .arg(entry_id_read_after).query_async(con)
            .await.unwrap();

    srr.keys.into_iter().map(|stream_key| {
        stream_key.ids.into_iter().map(|entry| {
            let id = entry.id;
            let v = redis::from_redis_value::<R>(&entry.map.get(entry_field).unwrap()).unwrap();
            (id, v)
        }).collect::<Vec<(String, R)>>()
    }).flatten().collect()
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
struct TestEntry {
    test_data: TestData,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
struct TestData {
    v: i32,
}

impl FromRedisValue for TestData {
    fn from_redis_value(v: &redis::Value) -> RedisResult<Self> {
        let s: String = redis::from_redis_value(v)?;
        Ok(serde_json::from_str(s.as_str()).unwrap())
    }
}

use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub static PURCHASE_DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();

pub async fn create_purchase_db_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://moke:moke@localhost:15432/purchase")
        .await
        .unwrap()
}

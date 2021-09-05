use sqlx::{Executor, Postgres};

pub async fn insert<'e, Exec>(executor: Exec, record: PurchaseItemRecord) -> anyhow::Result<u64>
    where
        Exec: Executor<'e, Database=Postgres>,
{
    let query = "INSERT INTO purchase_item VALUES ($1, $2, $3)";

    let result =
        sqlx::query(query)
            .bind(record.purchase_id)
            .bind(record.item_id)
            .bind(record.quantity)
            .execute(executor)
            .await?;

    Ok(result.rows_affected())
}

pub struct PurchaseItemRecord {
    pub purchase_id: String,
    pub item_id: String,
    pub quantity: i32,
}
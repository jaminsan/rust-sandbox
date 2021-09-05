use sqlx::{Executor, Postgres};

pub async fn insert<'e, Exec>(executor: Exec, record: PurchaseRecord) -> anyhow::Result<u64>
    where
        Exec: Executor<'e, Database=Postgres>,
{
    let query = "INSERT INTO purchase VALUES ($1, $2)";

    let result =
        sqlx::query(query)
            .bind(record.purchase_id)
            .bind(record.customer_id) // TODO: autocomplete 効かなくなる
            .execute(executor)
            .await?;

    Ok(result.rows_affected())
}

pub struct PurchaseRecord {
    pub purchase_id: String,
    pub customer_id: String,
}

use sqlx::{Executor, Postgres, Row};

pub async fn find_by_item_id<'e, Exec>(executor: Exec, item_id: String) -> anyhow::Result<Option<ItemStockRecord>>
    where
        Exec: Executor<'e, Database=Postgres> {
    let query = "SELECT * FROM item_stock WHERE item_id = $1";

    let result =
        sqlx::query(query)
            .bind(item_id)
            .fetch_optional(executor)
            .await?;

    if result.is_none() {
        return Ok(None)
    }

    let row = result.unwrap();
    Ok(Some(ItemStockRecord {
        item_id: row.try_get("item_id")?,
        quantity: row.try_get("quantity")?,
    }))
}

pub async fn update<'e, Exec>(executor: Exec, record: ItemStockRecord) -> anyhow::Result<u64>
    where
        Exec: Executor<'e, Database=Postgres> {
    let query = "UPDATE item_stock SET quantity = $1 WHERE item_id = $2";

    let result =
        sqlx::query(query)
            .bind(record.quantity)
            .bind(record.item_id)
            .execute(executor)
            .await?;

    Ok(result.rows_affected())
}


pub struct ItemStockRecord {
    pub item_id: String,
    pub quantity: i32,
}
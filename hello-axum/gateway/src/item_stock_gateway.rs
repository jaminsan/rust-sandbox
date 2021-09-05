use sqlx::{Pool, Postgres};
use ulid::Ulid;

use domain::item::{ItemId, ItemStock, StockQuantity};
use driver::item_stock_table;
use driver::item_stock_table::ItemStockRecord;
use port::item_stock_port::{FindItemStock, UpdateItemStock};

pub struct ItemStockGateway<'p> {
    pub pool: &'p Pool<Postgres>,
}

#[async_trait::async_trait]
impl FindItemStock for ItemStockGateway<'_> {
    async fn run(&self, item_id: &ItemId) -> anyhow::Result<Option<ItemStock>> {
        let maybe_item_stock_record =
            item_stock_table::find_by_item_id(self.pool, item_id.0.to_string()).await?;

        Ok(maybe_item_stock_record.map(convert_to_item_stock))
    }
}

#[async_trait::async_trait]
impl UpdateItemStock for ItemStockGateway<'_> {
    async fn run(&self, item_stock: &ItemStock) -> anyhow::Result<()> {
        let item_stock_record = convert_to_item_stock_record(item_stock);

        item_stock_table::update(self.pool, item_stock_record).await?;

        Ok(())
    }
}

fn convert_to_item_stock(item_stock_record: ItemStockRecord) -> ItemStock {
    ItemStock {
        item_id: ItemId(Ulid::from_string(&item_stock_record.item_id.to_string()).unwrap()),
        quantity: StockQuantity(item_stock_record.quantity),
    }
}

fn convert_to_item_stock_record(item_stock: &ItemStock) -> ItemStockRecord {
    ItemStockRecord {
        item_id: item_stock.item_id.0.to_string(),
        quantity: item_stock.quantity.0,
    }
}
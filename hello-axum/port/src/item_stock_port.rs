use domain::item::{ItemId, ItemStock};

#[async_trait::async_trait]
pub trait FindItemStock {
    async fn run(&self, item_id: &ItemId) -> anyhow::Result<Option<ItemStock>>;
}

#[async_trait::async_trait]
pub trait UpdateItemStock {
    async fn run(&self, item_stock: &ItemStock) -> anyhow::Result<()>;
}

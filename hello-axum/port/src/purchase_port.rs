use domain::purchase;

#[async_trait::async_trait]
pub trait SavePurchase {
    async fn run(&self, purchase: &purchase::Purchase) -> anyhow::Result<()>;
}

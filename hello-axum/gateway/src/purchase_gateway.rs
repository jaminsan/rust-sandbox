use sqlx::{Pool, Postgres};

use domain::purchase::{Purchase, PurchaseId, PurchaseItem};
use driver::{purchase_item_table, purchase_table};
use driver::purchase_item_table::PurchaseItemRecord;
use driver::purchase_table::PurchaseRecord;
use port::purchase_port::SavePurchase;

pub struct PurchaseGateway<'p> {
    pub pool: &'p Pool<Postgres>,
}

#[async_trait::async_trait]
impl SavePurchase for PurchaseGateway<'_> {
    async fn run(&self, purchase: &Purchase) -> anyhow::Result<()> {
        let mut tx = self.pool.begin().await?;

        let purchase_record = convert_to_purchase_record(&purchase);
        purchase_table::insert(&mut tx, purchase_record).await?;

        let purchase_item_record = convert_to_purchase_item_record(&purchase.purchase_id, &purchase.item);
        purchase_item_table::insert(&mut tx, purchase_item_record).await?;

        tx.commit().await?;

        Ok(())
    }
}

fn convert_to_purchase_record(p: &Purchase) -> PurchaseRecord {
    PurchaseRecord {
        purchase_id: p.purchase_id.0.to_string(),
        customer_id: p.customer_id.0.to_string(),
    }
}

fn convert_to_purchase_item_record(pid: &PurchaseId, pi: &PurchaseItem) -> PurchaseItemRecord {
    PurchaseItemRecord {
        purchase_id: pid.0.to_string(),
        item_id: pi.item_id.0.to_string(),
        quantity: pi.quantity.0,
    }
}

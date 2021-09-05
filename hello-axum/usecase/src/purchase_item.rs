use ulid::Ulid;

use domain::customer::CustomerId;
use domain::item::ItemId;
use domain::purchase::{Purchase, PurchaseId, PurchaseItem, PurchaseQuantity};
use port::item_stock_port::{FindItemStock, UpdateItemStock};
use port::purchase_port::SavePurchase;

use crate::purchase_item::PurchaseItemError::{ItemOutOfStock, ItemStockNotFound};

pub async fn execute(customer_id: CustomerId,
                     item_id: ItemId,
                     quantity: PurchaseQuantity,
                     save_purchase: &impl SavePurchase,
                     find_item_stock: &impl FindItemStock,
                     save_item_stock: &impl UpdateItemStock,
) -> anyhow::Result<()> {
    // TODO: minimize lock cost or implement lock free logic
    // TODO: transaction

    let item_stock =
        find_item_stock.run(&item_id)
            .await?
            .ok_or(ItemStockNotFound { item_id: item_id.clone() })?;

    let item_stock =
        item_stock
            .sub(quantity)
            .ok_or(ItemOutOfStock { item_id: item_id.clone(), quantity: quantity.clone() })?;

    let purchase =
        Purchase {
            purchase_id: PurchaseId(Ulid::new()),
            customer_id,
            item: PurchaseItem { item_id, quantity },
        };

    save_item_stock.run(&item_stock).await?;
    save_purchase.run(&purchase).await?;

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum PurchaseItemError {
    #[error("Item stock not found item_id={item_id:?}. Item may end of sales, or invalid item_id specified.")]
    ItemStockNotFound { item_id: ItemId },

    #[error("Item stock already out of stock item_id={item_id:?} quantity={quantity:?}.")]
    ItemOutOfStock { item_id: ItemId, quantity: PurchaseQuantity },
}

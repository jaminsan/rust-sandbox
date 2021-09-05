use crate::purchase::PurchaseQuantity;

#[derive(Debug, Copy, Clone)]
pub struct ItemStock {
    pub item_id: ItemId,
    pub quantity: StockQuantity,
}

impl ItemStock {
    pub fn sub(self, purchase_quantity: PurchaseQuantity) -> Option<ItemStock> {
        if purchase_quantity.0 <= self.quantity.0 {
            Some(ItemStock { item_id: self.item_id, quantity: StockQuantity(self.quantity.0 - purchase_quantity.0) })
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemId(pub ulid::Ulid);

#[derive(Debug, Copy, Clone)]
pub struct StockQuantity(pub i32);

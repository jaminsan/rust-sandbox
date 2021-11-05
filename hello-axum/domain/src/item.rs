use std::cmp::Ordering;

use crate::purchase::PurchaseQuantity;

#[derive(Debug, Copy, Clone)]
pub struct ItemStock {
    pub item_id: ItemId,
    pub quantity: StockQuantity,
}

impl ItemStock {
    pub fn sub(self, purchase_quantity: PurchaseQuantity) -> Option<ItemStock> {
        if purchase_quantity <= self.quantity {
            Some(ItemStock { item_id: self.item_id, quantity: StockQuantity(self.quantity.0 - purchase_quantity.0) })
        } else {
            None
        }
    }
}

impl PartialEq<StockQuantity> for PurchaseQuantity {
    fn eq(&self, other: &StockQuantity) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::PartialOrd<StockQuantity> for PurchaseQuantity {
    fn partial_cmp(&self, other: &StockQuantity) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }

    fn lt(&self, other: &StockQuantity) -> bool {
        self.0 < other.0
    }

    fn le(&self, other: &StockQuantity) -> bool {
        self.0 <= other.0
    }

    fn gt(&self, other: &StockQuantity) -> bool {
        self.0 > other.0
    }

    fn ge(&self, other: &StockQuantity) -> bool {
        self.0 >= other.0
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ItemId(pub ulid::Ulid);

#[derive(Debug, Copy, Clone)]
pub struct StockQuantity(pub i32);

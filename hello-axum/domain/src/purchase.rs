use ulid::Ulid;

use crate::customer::CustomerId;
use crate::item::ItemId;

#[derive(Debug, Copy, Clone)]
pub struct Purchase {
    pub purchase_id: PurchaseId,
    pub customer_id: CustomerId,
    pub item: PurchaseItem,
}

#[derive(Debug, Copy, Clone)]
pub struct PurchaseId(pub Ulid);

#[derive(Debug, Copy, Clone)]
pub struct PurchaseItem {
    pub item_id: ItemId,
    pub quantity: PurchaseQuantity,
}

#[derive(Debug, Copy, Clone)]
pub struct PurchaseQuantity(pub i32);
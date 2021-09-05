use axum::http::Response;
use axum::prelude::*;
use axum::response::IntoResponse;
use ulid::Ulid;

use domain::customer::CustomerId;
use domain::item::ItemId;
use domain::purchase::PurchaseQuantity;
use gateway::item_stock_gateway::ItemStockGateway;
use gateway::purchase_gateway::PurchaseGateway;
use usecase;

use crate::helper::PURCHASE_DB_POOL;
use usecase::purchase_item::PurchaseItemError;

pub async fn get_purchase(extract::Path(purchase_id): extract::Path<String>) -> String {
    todo!()
}

pub async fn post_purchase(body: extract::Json<PostPurchaseRequest>) -> impl IntoResponse {
    let purchase_gateway = PurchaseGateway { pool: &PURCHASE_DB_POOL.get().unwrap() };
    let item_stock_gateway = ItemStockGateway { pool: &PURCHASE_DB_POOL.get().unwrap() };
    let result =
        usecase::purchase_item::execute(CustomerId(unsafe_to_ulid(&body.customer_id)),
                                        ItemId(unsafe_to_ulid(&body.item_id)),
                                        PurchaseQuantity(body.quantity),
                                        &purchase_gateway,
                                        &item_stock_gateway,
                                        &item_stock_gateway,
        ).await;

    match result {
        Ok(_) => {
            Response::new(Body::empty())
        }
        Err(err) => {
            match err.downcast_ref::<PurchaseItemError>() {
                None => {
                    tracing::error!("{:?}", &err);
                    crate::handler::helper::internal_server_error()
                }
                Some(PurchaseItemError::ItemStockNotFound { item_id }) => {
                    crate::handler::helper::bad_request()
                }
                Some(PurchaseItemError::ItemOutOfStock { item_id, quantity }) => {
                    crate::handler::helper::bad_request()
                }
            }
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostPurchaseRequest {
    customer_id: String,
    item_id: String,
    quantity: i32,
}

fn unsafe_to_ulid(s: &String) -> Ulid {
    ulid::Ulid::from_string(&s.to_owned()).unwrap()
}

extern crate domain;
extern crate gateway;
extern crate usecase;

use std::net::SocketAddr;

use axum::prelude::*;
use axum::route;

use handler::purchase;

use crate::helper::{create_purchase_db_pool, PURCHASE_DB_POOL};

mod handler;
mod helper;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    PURCHASE_DB_POOL.set(create_purchase_db_pool().await).unwrap();

    let routes =
        route("/purchases", post(purchase::post_purchase))
            .route("/purchases/:purchase_id", get(purchase::get_purchase));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("application started on port {}", addr);

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

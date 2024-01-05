use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

use super::order_db::DBTableTrait;
use crate::{
    common::models::restaurant_schema::{CookStatus, Order, TableResponse},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateOrdersRequest {
    pub orders: Vec<String>,
}

#[derive(Serialize)]
struct CreateOrdersResponse {
    table: TableResponse,
}

pub async fn create_order(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<i64>,
    Json(create_order_request): Json<CreateOrdersRequest>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let mut order_id = app_state.orders.lock().await;

    //validate that the table exists
    match db.get_table(&table_id).await {
        Ok(table) => table,
        Err(e) => {
            todo!();
        }
    };

    let mut order_docs = vec![];
    //prepare the orders
    for item_name in create_order_request.orders.into_iter() {
        let item = match db.get_item(item_name).await {
            Ok(opt_item) => opt_item,
            Err(e) => todo!(),
        };
        match item {
            Some(item) => {
                *order_id += 1;
                let order = Order {
                    order_id: *order_id,
                    table_id,
                    ordered_time: Utc::now(),
                    cook_status: CookStatus::InProgress,
                    item,
                };
                match mongodb::bson::to_document(&order) {
                    Ok(document) => order_docs.push(document),
                    Err(e) => panic!("something didn't work!"),
                }
            }
            None => continue,
        }
    }

    //insert the order into the table
    match db.create_orders(&table_id, order_docs).await {
        Ok(table) => (
            StatusCode::OK,
            Json(CreateOrdersResponse {
                table: table.into(),
            }),
        ),
        Err(e) => todo!(),
    }
}

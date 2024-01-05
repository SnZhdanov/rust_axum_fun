use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use axum_extra::extract::Query as ExtraQuery;

use chrono::Utc;

use serde::{Deserialize, Serialize};

use super::order_db::DBTableTrait;
use crate::{
    common::models::{
        pagination_schema::Pagination,
        restaurant_schema::{CookStatus, Order, OrderResponse, TableResponse},
    },
    AppState,
};

#[derive(Deserialize)]
pub struct CreateOrdersRequest {
    pub orders: Vec<String>,
}

#[derive(Serialize)]
pub struct ReturnTableResponse {
    pub table: TableResponse,
}

#[derive(Serialize)]
pub struct ListTableOrdersResponse {
    pub table_id: i64,
    pub orders: Vec<OrderResponse>,
}

#[derive(Serialize)]
pub struct GetOrderResponse {
    pub order: OrderResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListOrderFiltersRequest {
    #[serde(default = "default_vec_i64")]
    pub table_ids: Vec<i64>,
    #[serde(default = "default_vec_strings")]
    pub item_names: Vec<String>,
    pub cook_status: Option<CookStatus>,
}
pub fn default_vec_i64() -> Vec<i64> {
    vec![]
}

pub fn default_vec_strings() -> Vec<String> {
    vec![]
}

#[derive(Serialize)]
pub struct ListOrdersResponse {
    pub orders: Vec<OrderResponse>,
    pub pagination: ListOrderPaginationResponse,
    pub filters: ListOrderFiltersRequest,
}

#[derive(Serialize)]
pub struct ListOrderPaginationResponse {
    pub total: u64,
    pub limit: i64,
    pub offset: u64,
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
            Json(ReturnTableResponse {
                table: table.into(),
            }),
        ),
        Err(e) => todo!(),
    }
}

pub async fn get_order(
    State(app_state): State<Arc<AppState>>,
    Path((table_id, order_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let db = &app_state.db;

    match db.get_order(&table_id, &order_id).await {
        Ok(order) => (
            StatusCode::OK,
            Json(GetOrderResponse {
                order: order.into(),
            }),
        ),
        Err(e) => todo!(),
    }
}

pub async fn list_all_orders(
    State(app_state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
    filters: ExtraQuery<ListOrderFiltersRequest>,
) -> impl IntoResponse {
    let db = &app_state.db;

    let pagination = Pagination {
        limit: pagination.limit,
        offset: pagination.offset,
    };

    let filters = ListOrderFiltersRequest {
        table_ids: filters.table_ids.clone(),
        item_names: filters.item_names.clone(),
        cook_status: filters.cook_status.clone(),
    };

    match db.list_all_orders(&filters).await {
        Ok(mut list_order_result) => {
            list_order_result.orders = match &filters.cook_status {
                Some(cook_status) => {
                    handle_cooking_status_filter(list_order_result.orders, cook_status)
                }
                None => list_order_result.orders,
            };

            let total = list_order_result.orders.len() as u64;
            //now handle pagination since it does nothing right now.

            list_order_result.orders = handle_pagination(list_order_result.orders, &pagination);
            (
                StatusCode::OK,
                Json(ListOrdersResponse {
                    orders: list_order_result.orders,
                    pagination: ListOrderPaginationResponse {
                        total,
                        limit: pagination.limit,
                        offset: pagination.offset,
                    },
                    filters,
                }),
            )
        }
        Err(e) => {
            todo!();
        }
    }
}

pub async fn delete_order(
    State(app_state): State<Arc<AppState>>,
    Path((table_id, order_id)): Path<(i64, i64)>,
) -> impl IntoResponse {
    let db = &app_state.db;

    match db.delete_order(&table_id, &order_id).await {
        Ok(table) => (
            StatusCode::OK,
            Json(ReturnTableResponse {
                table: table.into(),
            }),
        ),
        Err(e) => todo!(),
    }
}

pub fn handle_cooking_status_filter(
    orders: Vec<OrderResponse>,
    cook_status: &CookStatus,
) -> Vec<OrderResponse> {
    orders
        .into_iter()
        .filter(|order| &order.cook_status == cook_status)
        .collect::<Vec<OrderResponse>>()
}

pub fn handle_pagination(
    orders: Vec<OrderResponse>,
    pagination: &Pagination,
) -> Vec<OrderResponse> {
    let total = orders.len();
    if pagination.offset >= total as u64 || pagination.limit == 0 {
        return [].to_vec();
    }
    let orders_as_slice = orders.as_slice();
    if (pagination.offset as i64 + pagination.limit) > total as i64 {
        let sliced_vec = &orders_as_slice[pagination.offset as usize..total];
        sliced_vec.to_vec()
    } else {
        let sliced_vec = &orders_as_slice
            [pagination.offset as usize..(pagination.offset as usize + pagination.limit as usize)];
        sliced_vec.to_vec()
    }
}

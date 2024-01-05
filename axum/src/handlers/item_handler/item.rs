use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};

use crate::{
    common::models::{pagination_schema::Pagination, restaurant_schema::ItemResponse},
    AppState,
};

use super::item_handler::DBTableTrait;

#[derive(Deserialize, Serialize)]
pub struct ListItemsRequest {
    #[serde(default = "empty_vec_of_strings")]
    pub item_names: Vec<String>,
}

pub fn empty_vec_of_strings() -> Vec<String> {
    [].to_vec()
}

#[derive(Serialize)]
struct ListItemsResponse {
    items: Vec<ItemResponse>,
    pagination: ListItemsPaginationResponse,
    filters: ListItemsRequest,
    errors: ListItemsErrorResponse,
}

#[derive(Serialize)]
struct ListItemsPaginationResponse {
    total: u64,
    limit: i64,
    offset: u64,
}

#[derive(Serialize)]
struct ListItemsErrorResponse {
    failed_items_ids: Option<Vec<String>>,
    failed_items_count: u64,
}

pub async fn list_items(
    State(app_state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
    Query(filters): Query<ListItemsRequest>,
) -> impl IntoResponse {
    let db = &app_state.db;

    let pagination = Pagination {
        limit: pagination.limit,
        offset: pagination.offset,
    };

    match db.list_items(filters.item_names.clone(), &pagination).await {
        Ok(list_results) => (
            StatusCode::OK,
            Json(ListItemsResponse {
                items: list_results.items,
                filters: filters,
                pagination: ListItemsPaginationResponse {
                    total: list_results.count,
                    limit: pagination.limit,
                    offset: pagination.offset,
                },
                errors: ListItemsErrorResponse {
                    failed_items_ids: list_results.failed_items,
                    failed_items_count: list_results.dropped,
                },
            }),
        ),
        Err(e) => {
            todo!()
        }
    }
    // match db.get_order(&table_id, &order_id).await {
    //     Ok(order) => (
    //         StatusCode::OK,
    //         Json(GetOrderResponse {
    //             order: order.into(),
    //         }),
    //     ),
    //     Err(e) => todo!(),
    // }
}

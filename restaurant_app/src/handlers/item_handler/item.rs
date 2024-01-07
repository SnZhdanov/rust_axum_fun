use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::Query;
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        errors::AxumErrorResponse,
        models::{pagination_schema::Pagination, restaurant_schema::ItemResponse},
    },
    AppState,
};

use super::item_db::DBTableTrait;

#[derive(Deserialize, Serialize)]
pub struct ListItemsRequest {
    #[serde(default = "empty_vec_of_strings")]
    pub item_names: Vec<String>,
}

pub fn empty_vec_of_strings() -> Vec<String> {
    [].to_vec()
}

#[derive(Serialize)]
pub struct ListItemsResponse {
    pub items: Vec<ItemResponse>,
    pub pagination: ListItemsPaginationResponse,
    pub filters: ListItemsRequest,
    pub errors: ListItemsErrorResponse,
}

#[derive(Serialize)]
pub struct ListItemsPaginationResponse {
    pub total: u64,
    pub limit: i64,
    pub offset: u64,
}

#[derive(Serialize)]
pub struct ListItemsErrorResponse {
    pub failed_items_ids: Option<Vec<String>>,
    pub failed_items_count: u64,
}

pub async fn list_items(
    State(app_state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
    Query(filters): Query<ListItemsRequest>,
) -> Result<(StatusCode, Json<ListItemsResponse>), (StatusCode, Json<AxumErrorResponse>)> {
    let db = &app_state.db;

    let pagination = Pagination {
        limit: pagination.limit,
        offset: pagination.offset,
    };

    match db.list_items(filters.item_names.clone(), &pagination).await {
        Ok(list_results) => Ok((
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
        )),
        Err(e) => Err(e.to_axum_error()),
    }
}

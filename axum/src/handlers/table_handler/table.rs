use std::sync::Arc;

use axum::{
    extract::Path, extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json,
};
use axum_extra::extract::Query as ExtraQuery;
use serde::{Deserialize, Serialize};

use crate::{
    common::models::{
        pagination_schema::Pagination,
        restaurant_schema::{CookStatus, Table, TableResponse},
    },
    AppState,
};

use super::table_db::DBTableTrait;

#[derive(Deserialize, Serialize)]
struct PostTableResponse {
    id: String,
    table: TableResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListTableFiltersRequest {
    pub table_id: Option<i64>,
    pub order_id: Option<i64>,
    pub item_name: Option<String>,
    #[serde(default = "default_vec_strings")]
    pub item_names: Vec<String>,
}

pub fn default_vec_strings() -> Vec<String>{
    vec![]
}


#[derive(Serialize)]
struct ListTableResponse {
    tables: Vec<TableResponse>,
    pagination: ListTablePaginationResponse,
    filters: ListTableFiltersRequest,
    errors: ListTableErrorResponse,
}

#[derive(Serialize)]
struct ListTablePaginationResponse {
    total: u64,
    limit: i64,
    offset: u64,
}

#[derive(Serialize)]
struct ListTableErrorResponse {
    failed_table_ids: Option<Vec<String>>,
    failed_table_count: u64,
}

#[derive(Serialize)]
struct DeleteTableResponse {
    table: TableResponse,
}

pub async fn create_table(State(app_state): State<Arc<AppState>>) -> impl IntoResponse {
    let mut tables = app_state.tables.lock().await;
    let db = &app_state.db;
    *tables += 1;

    let table = Table {
        id: mongodb::bson::oid::ObjectId::new().to_hex(),
        table_id: *tables,
        orders: vec![].into(),
    };

    match db.create_table(&table).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(PostTableResponse {
                id: table.id.clone(),
                table: table.into(),
            }),
        ),
        Err(e) => {
            todo!()
        }
    }
}

pub async fn list_table(
    State(app_state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
    filters: ExtraQuery<ListTableFiltersRequest>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let pagination = Pagination {
        limit: pagination.limit,
        offset: pagination.offset,
    };

    let filters = ListTableFiltersRequest {
        table_id: filters.table_id,
        order_id: filters.order_id,
        item_name: filters.item_name.clone(),
        item_names: filters.item_names.clone(),
    };

    match db.list_tables(&pagination, filters.clone()).await {
        Ok(list_result) => (
            StatusCode::OK,
            Json(ListTableResponse {
                tables: list_result.tables,
                filters,
                pagination: ListTablePaginationResponse {
                    total: list_result.count,
                    limit: pagination.limit,
                    offset: pagination.offset,
                },
                errors: ListTableErrorResponse {
                    failed_table_ids: list_result.failed_tables,
                    failed_table_count: list_result.dropped,
                },
            }),
        ),
        Err(e) => todo!(),
    }
}

pub async fn get_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<i64>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let table_id = table_id;

    match db.get_table(table_id).await {
        Ok(table_result) => (StatusCode::OK, Json(TableResponse::from(table_result))),
        Err(e) => todo!(),
    }
}

pub async fn delete_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<i64>,
) -> impl IntoResponse {
    let db = &app_state.db;
    let table_id = table_id;

    match db.delete_table(table_id).await {
        Ok(table_response) => (
            StatusCode::OK,
            Json(DeleteTableResponse {
                table: table_response,
            }),
        ),
        Err(e) => todo!(),
    }
}

use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
//use futures::TryStreamExt;

use axum::{
    extract::Path, extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json,
};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{
    common::models::{
        pagination_schema::Pagination,
        restaurant_schema::{Order, Table, TableResponse},
    },
    AppState,
};

use super::table_db::{DBTableTrait, ListTablesResult};

#[derive(Deserialize, Serialize)]
struct PostTableResponse {
    id: String,
    table: TableResponse,
}

#[derive(Serialize)]
struct ListTableResponse {
    tables: Vec<TableResponse>,
    pagination: ListTablePaginationResponse,
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
) -> impl IntoResponse {
    let db = &app_state.db;
    let pagination = Pagination {
        limit: pagination.limit,
        offset: pagination.offset,
    };

    match db.list_tables(&pagination, &None).await {
        Ok(list_result) => (
            StatusCode::OK,
            Json(ListTableResponse {
                tables: list_result.tables,
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
        Ok(list_result) => (StatusCode::OK, Json(list_result)),
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

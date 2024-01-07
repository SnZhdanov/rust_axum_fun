use std::sync::Arc;

use axum::{extract::Path, extract::Query, extract::State, http::StatusCode, Json};
use axum_extra::extract::Query as ExtraQuery;

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        errors::AxumErrorResponse,
        models::{
            pagination_schema::Pagination,
            restaurant_schema::{CookStatus, Order, Table, TableResponse},
        },
    },
    AppState,
};

use super::table_db::DBTableTrait;

#[derive(Deserialize, Serialize)]
pub struct PostTableResponse {
    pub id: String,
    pub table: TableResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ListTableFiltersRequest {
    pub table_id: Option<i64>,
    pub order_id: Option<i64>,
    pub item_name: Option<String>,
    #[serde(default = "default_vec_strings")]
    pub item_names: Vec<String>,
}

pub fn default_vec_strings() -> Vec<String> {
    vec![]
}

#[derive(Deserialize, Serialize)]
pub struct ListTableResponse {
    pub tables: Vec<TableResponse>,
    pub pagination: ListTablePaginationResponse,
    pub filters: ListTableFiltersRequest,
    pub errors: ListTableErrorResponse,
}

#[derive(Deserialize, Serialize)]
pub struct ListTablePaginationResponse {
    pub total: u64,
    pub limit: i64,
    pub offset: u64,
}

#[derive(Deserialize, Serialize)]
pub struct ListTableErrorResponse {
    pub failed_table_ids: Option<Vec<String>>,
    pub failed_table_count: u64,
}

#[derive(Deserialize, Serialize)]
pub struct DeleteTableResponse {
    pub table: TableResponse,
}

#[derive(Deserialize, Serialize)]
pub struct CreateTableOrdersRequest {
    #[serde(default = "default_vec_strings")]
    pub orders: Vec<String>,
}

pub async fn create_table(
    State(app_state): State<Arc<AppState>>,
    Json(create_order_request): Json<CreateTableOrdersRequest>,
) -> Result<(StatusCode, Json<PostTableResponse>), (StatusCode, Json<AxumErrorResponse>)> {
    let mut table_id = app_state.tables.lock().await;
    let mut order_id = app_state.orders.lock().await;

    let db = &app_state.db;

    let mut order_added_counter = 0;

    let mut orders: Vec<Order> = [].to_vec();

    //if items are inside the order, then let's get the item and custom make orders
    for item_name in create_order_request.orders.into_iter() {
        let item = match db.get_item_table(item_name).await {
            Ok(opt_item) => opt_item,
            Err(e) => return Err(e.to_axum_error()),
        };
        match item {
            Some(item) => {
                order_added_counter += 1;
                orders.push(Order {
                    order_id: *order_id + order_added_counter,
                    table_id: *table_id + 1,
                    ordered_time: Utc::now(),
                    cook_status: CookStatus::InProgress,
                    item,
                })
            }
            None => continue,
        }
    }

    let table = Table {
        id: mongodb::bson::oid::ObjectId::new().to_hex(),
        table_id: *table_id + 1,
        orders,
    };

    match db.create_table(&table).await {
        Ok(_) => {
            *table_id += 1;
            *order_id += order_added_counter;
            Ok((
                StatusCode::CREATED,
                Json(PostTableResponse {
                    id: table.id.clone(),
                    table: table.into(),
                }),
            ))
        }
        Err(e) => Err(e.to_axum_error()),
    }
}

pub async fn list_table(
    State(app_state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
    filters: ExtraQuery<ListTableFiltersRequest>,
) -> Result<(StatusCode, Json<ListTableResponse>), (StatusCode, Json<AxumErrorResponse>)> {
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
        Ok(list_result) => Ok((
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
        )),
        Err(e) => Err(e.to_axum_error()),
    }
}

pub async fn get_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<i64>,
) -> Result<(StatusCode, Json<TableResponse>), (StatusCode, Json<AxumErrorResponse>)> {
    let db = &app_state.db;
    let table_id = table_id;

    match db.get_table(table_id).await {
        Ok(table_result) => Ok((StatusCode::OK, Json(TableResponse::from(table_result)))),
        Err(e) => Err(e.to_axum_error()),
    }
}

pub async fn delete_table(
    State(app_state): State<Arc<AppState>>,
    Path(table_id): Path<i64>,
) -> Result<(StatusCode, Json<DeleteTableResponse>), (StatusCode, Json<AxumErrorResponse>)> {
    let db = &app_state.db;
    let table_id = table_id;

    match db.delete_table(table_id).await {
        Ok(table_response) => Ok((
            StatusCode::OK,
            Json(DeleteTableResponse {
                table: table_response,
            }),
        )),
        Err(e) => Err(e.to_axum_error()),
    }
}

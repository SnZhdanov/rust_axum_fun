use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
//use futures::TryStreamExt;


use axum::{http::StatusCode, response::IntoResponse, Json, extract::State};
use serde_json::json;
use tokio::sync::Mutex;

use crate::{common::models::restaurant_schema::{Table, Order}, AppState};

#[derive(Deserialize)]
struct TablePost{

}

#[derive(Deserialize, Serialize)]
struct PostTableResponse{
    tables: u64
}
pub async fn create_table(State(app_state): State<Arc<AppState>>) 
-> impl IntoResponse{
    let mut tables = app_state.tables.lock().await;
    let db = &app_state.db;
    *tables +=1;
    (StatusCode::CREATED,    
        Json(PostTableResponse{
            tables: *tables
    }))


}
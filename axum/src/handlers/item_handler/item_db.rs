use async_trait::async_trait;
use axum::http::StatusCode;
use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::common::{
    database,
    database_helpers::collect_cursor,
    errors::{AxumErrors, ErrorResponse},
    models::{
        pagination_schema::Pagination,
        restaurant_schema::{Item, ItemResponse},
    },
};

#[derive(Serialize, Deserialize, Clone)]
pub struct ListItemResults {
    pub items: Vec<ItemResponse>,
    pub failed_items: Option<Vec<String>>,
    pub count: u64,
    pub dropped: u64,
}

#[async_trait]
pub trait DBTableTrait {
    async fn list_items(
        &self,
        item_names: Vec<String>,
        pagination: &Pagination,
    ) -> Result<ListItemResults, ErrorResponse>;
}

#[faux::methods]
#[async_trait]
impl DBTableTrait for database::DB {
    async fn list_items(
        &self,
        item_names: Vec<String>,
        pagination: &Pagination,
    ) -> Result<ListItemResults, ErrorResponse> {
        let item_collection = self
            .db
            .database("item_management")
            .collection::<Document>("items");
        let filter = match item_names.is_empty() {
            true => {
                doc! {}
            }
            false => {
                doc! {
                    "item_name":{
                        "$in":item_names
                    }
                }
            }
        };

        let find_options = mongodb::options::FindOptions::builder()
            .limit(pagination.limit)
            .skip(pagination.offset)
            .sort(None)
            .build();

        let count_options = mongodb::options::CountOptions::builder().build();

        let count = match item_collection
            .count_documents(filter.clone(), Some(count_options))
            .await
        {
            Ok(count) => count,
            Err(e) => {
                error!("Unexpected error occured while coutning Items in the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        };

        match item_collection.find(filter, find_options).await {
            Ok(cursor) => {
                let (items, failed_items, dropped) =
                    match collect_cursor::<Item, ItemResponse>(cursor).await {
                        Ok(collect_cursor_result) => collect_cursor_result.get_results(),
                        Err(e) => {
                            return Err(ErrorResponse {
                                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                                error: e.into(),
                            });
                        }
                    };

                Ok(ListItemResults {
                    items,
                    failed_items: match failed_items.len() {
                        0 => None,
                        _ => Some(failed_items),
                    },
                    count,
                    dropped: dropped,
                })
            }
            Err(e) => {
                error!("Unexpected error occured while coutning Items in the Database. Error: {e}");
                Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                })
            }
        }
    }
}

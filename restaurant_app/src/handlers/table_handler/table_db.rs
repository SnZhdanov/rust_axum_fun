use async_trait::async_trait;

use mongodb::bson::doc;
use mongodb::bson::Document;
use mongodb::bson::Regex;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::common::database;
use crate::common::database_helpers::collect_cursor;
use crate::common::errors::AxumErrors;
use crate::common::errors::ErrorResponse;
use crate::common::models::pagination_schema::Pagination;
use crate::common::models::restaurant_schema::Item;
use crate::common::models::restaurant_schema::{Table, TableResponse};
use crate::table_handler::table::ListTableFiltersRequest;
use axum::http::StatusCode;
#[derive(Serialize, Deserialize, Clone)]
pub struct ListTablesResult {
    pub tables: Vec<TableResponse>,
    pub failed_tables: Option<Vec<String>>,
    pub count: u64,
    pub dropped: u64,
}

#[derive(Serialize, Clone)]
pub struct ListTableFiltersBson {
    #[serde(skip_serializing_if = "Option::is_none")]
    table_id: Option<i64>,
    #[serde(rename = "orders.order_id", skip_serializing_if = "Option::is_none")]
    order_id: Option<i64>,
    #[serde(
        rename = "orders.item.item_name",
        skip_serializing_if = "Option::is_none"
    )]
    item_name: Option<ItemNameFuzzyRegex>,
    #[serde(rename = "$and", skip_serializing_if = "Option::is_none")]
    item_names_strict: Option<Vec<ItemNameStrictCheck>>,
}

#[derive(Serialize, Clone)]
pub struct ItemNameFuzzyRegex {
    #[serde(rename(serialize = "$regex"))]
    regex: Regex,
}

#[derive(Serialize, Clone)]
pub struct ItemNameStrictCheck {
    #[serde(rename = "orders.item.item_name")]
    item_name: String,
}

impl From<ListTableFiltersRequest> for ListTableFiltersBson {
    fn from(filters: ListTableFiltersRequest) -> Self {
        Self {
            table_id: match filters.table_id {
                Some(t) => Some(t),
                None => None,
            },
            order_id: match filters.order_id {
                Some(o) => Some(o),
                None => None,
            },
            item_name: match filters.item_name.clone() {
                Some(item_name) => Some(ItemNameFuzzyRegex {
                    regex: Regex {
                        pattern: item_name,
                        options: "i".to_string(),
                    },
                }),
                None => None,
            },
            item_names_strict: match filters.item_names.is_empty() {
                true => None,
                false => Some(
                    filters
                        .item_names
                        .clone()
                        .into_iter()
                        .map(|item_name| ItemNameStrictCheck {
                            item_name: item_name,
                        })
                        .collect::<Vec<ItemNameStrictCheck>>(),
                ),
            },
        }
    }
}

#[async_trait]
pub trait DBTableTrait {
    async fn create_table(&self, table: &Table) -> Result<Table, ErrorResponse>;
    async fn get_table(&self, table_id: i64) -> Result<Table, ErrorResponse>;
    async fn list_tables(
        &self,
        pagination: &Pagination,
        filters: ListTableFiltersRequest,
    ) -> Result<ListTablesResult, ErrorResponse>;
    async fn delete_table(&self, table_id: i64) -> Result<TableResponse, ErrorResponse>;
    async fn get_item_table(&self, item_name: String) -> Result<Option<Item>, ErrorResponse>;
}

#[faux::methods]
#[async_trait]
impl DBTableTrait for database::DB {
    //this function is going to need session manager
    async fn create_table(&self, table: &Table) -> Result<Table, ErrorResponse> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //make sure the table doesn't exist then insert
        let filter = doc! {
            "table_id": &table.table_id as &i64
        };

        //first check if the id is in there
        match table_collection.find_one(filter.clone(), None).await {
            Ok(table) => match table {
                Some(table) => return Ok(table.into()),
                None => (),
            },
            Err(e) => {
                error!("Unexpected error occured while searching for the Table in the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        };

        // let table_as_bson = mongodb::bson::to_document(table).unwrap();

        match table_collection.insert_one(table, None).await {
            Ok(_) => Ok(table.clone().into()),
            Err(e) => {
                error!("Unexpected error occured while inserting the Table into the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        }
    }
    async fn get_table(&self, table_id: i64) -> Result<Table, ErrorResponse> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        let filter = doc! {
            "table_id": table_id
        };

        match table_collection.find_one(filter, None).await {
            Ok(opt_table) => match opt_table {
                Some(table) => Ok(table),
                None => {
                    return Err(ErrorResponse {
                        status_code: StatusCode::NOT_FOUND,
                        error: AxumErrors::NotFound.into(),
                    });
                }
            },
            Err(e) => {
                error!("Unexpected error occured while finding the Table from the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        }
    }

    async fn list_tables(
        &self,
        pagination: &Pagination,
        filters: ListTableFiltersRequest,
    ) -> Result<ListTablesResult, ErrorResponse> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Document>("tables");

        let find_options = mongodb::options::FindOptions::builder()
            .limit(pagination.limit)
            .skip(pagination.offset)
            .sort(None)
            .build();
        let filters_as_bson: ListTableFiltersBson = filters.into();

        let filter: Document = match mongodb::bson::to_document(&filters_as_bson) {
            Ok(document) => document,
            Err(e) => {
                error!("Unexpected error occured while deserializing the document! Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::BsonSerializeError.into(),
                });
            }
        };

        let count_options = mongodb::options::CountOptions::builder().build();

        let count = match table_collection
            .count_documents(filter.clone(), Some(count_options))
            .await
        {
            Ok(count) => count,
            Err(e) => {
                error!("Unexpected error occured while coutning the Tables from the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        };

        match table_collection.find(filter, find_options).await {
            Ok(cursor) => {
                let (tables, failed_tables, dropped) =
                    match collect_cursor::<Table, TableResponse>(cursor).await {
                        Ok(collect_cursor_result) => collect_cursor_result.get_results(),
                        Err(e) => {
                            return Err(ErrorResponse {
                                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                                error: e.into(),
                            })
                        }
                    };

                Ok(ListTablesResult {
                    tables,
                    failed_tables: match failed_tables.len() {
                        0 => None,
                        _ => Some(failed_tables),
                    },
                    count,
                    dropped: dropped,
                })
            }
            Err(e) => {
                error!("Unexpected error occured while Listing the Tables from the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        }
    }
    async fn delete_table(&self, table_id: i64) -> Result<TableResponse, ErrorResponse> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //get the table first then delete it
        let filter = doc! {
            "table_id": table_id
        };

        let table = match table_collection.find_one(filter.clone(), None).await {
            Ok(opt_table) => match opt_table {
                Some(table) => table,
                None => {
                    return Err(ErrorResponse {
                        status_code: StatusCode::NOT_FOUND,
                        error: AxumErrors::NotFound.into(),
                    })
                }
            },
            Err(e) => {
                error!("Unexpected error occured while searching the Table to delete from the Database. Error: {e}");
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        };

        match table_collection.delete_one(filter, None).await {
            Ok(_) => Ok(table.into()),
            Err(e) => {
                error!("Unexpected error occured while Deleting the Table from the Database. Error: {e}");
                Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                })
            }
        }
    }

    async fn get_item_table(&self, item_name: String) -> Result<Option<Item>, ErrorResponse> {
        let item_collection = self
            .db
            .database("item_management")
            .collection::<Item>("items");

        let filter = doc! {
            "item_name":item_name
        };

        match item_collection.find_one(filter, None).await {
            Ok(opt_item) => Ok(opt_item),
            Err(e) => {
                error!(
                    "Unexpected error occured while searching for Item in the Database. Error: {e}"
                );
                return Err(ErrorResponse {
                    status_code: StatusCode::INTERNAL_SERVER_ERROR,
                    error: AxumErrors::DBError.into(),
                });
            }
        }
    }
}

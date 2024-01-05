use async_trait::async_trait;
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use mongodb::bson::{doc, Document};
use mongodb::options::ReturnDocument;
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::common::database;
use crate::common::database_helpers::collect_cursor;

use crate::common::errors::AxumErrors;
use crate::common::models::restaurant_schema::{Item, Order, OrderResponse, Table};

use super::order::ListOrderFiltersRequest;

#[derive(Serialize)]
struct CreateOrderUpdateDoc {
    #[serde(rename = "$push")]
    push: Vec<Order>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Dat {
    ordered_time: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ListOrderResult {
    pub orders: Vec<OrderResponse>,
    pub failed_orders: Option<Vec<String>>,
    pub count: u64,
    pub dropped: u64,
}

#[async_trait]
pub trait DBTableTrait {
    async fn get_table(&self, table_id: &i64) -> Result<Table, (StatusCode, AxumErrors)>;
    async fn create_orders(
        &self,
        table_id: &i64,
        orders: Vec<Document>,
    ) -> Result<Table, (StatusCode, AxumErrors)>;
    async fn get_item(&self, item_name: String) -> Result<Option<Item>, (StatusCode, AxumErrors)>;
    async fn delete_order(
        &self,
        table_id: &i64,
        order_id: &i64,
    ) -> Result<Table, (StatusCode, AxumErrors)>;
    async fn get_order(
        &self,
        table_id: &i64,
        order_id: &i64,
    ) -> Result<Order, (StatusCode, AxumErrors)>;
    async fn list_table_orders(
        &self,
        table_id: &i64,
    ) -> Result<Vec<OrderResponse>, (StatusCode, AxumErrors)>;
    async fn list_all_orders(
        &self,
        filters: &ListOrderFiltersRequest,
    ) -> Result<ListOrderResult, (StatusCode, AxumErrors)>;
}

#[async_trait]
impl DBTableTrait for database::DB {
    async fn get_item(&self, item_name: String) -> Result<Option<Item>, (StatusCode, AxumErrors)> {
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
                return Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError));
            }
        }
    }

    async fn get_table(&self, table_id: &i64) -> Result<Table, (StatusCode, AxumErrors)> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //check if the table exists
        let filter = doc! {
            "table_id": table_id
        };

        match table_collection.find_one(filter, None).await {
            Ok(opt_table) => match opt_table {
                Some(table) => Ok(table),
                None => Err((StatusCode::NOT_FOUND, AxumErrors::NotFound)),
            },
            Err(e) => {
                error!("Unexpected error occured while searching for Table in the Database. Error: {e}");
                return Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError));
            }
        }
    }

    async fn create_orders(
        &self,
        table_id: &i64,
        orders: Vec<Document>,
    ) -> Result<Table, (StatusCode, AxumErrors)> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //check if the table exists
        let filter = doc! {
            "table_id": table_id
        };

        //upsert order
        let update = doc! {
            "$push":{
                "orders":{"$each": orders}
            }
        };

        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        match table_collection
            .find_one_and_update(filter, update, options)
            .await
        {
            Ok(opt_table) => match opt_table {
                Some(table) => Ok(table),
                None => Err((StatusCode::NOT_FOUND, AxumErrors::NotFound)),
            },
            Err(e) => {
                error!("Unexpected error occured while inserting Orders for Table in the Database. Error: {e}");
                Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError))
            }
        }
    }

    async fn delete_order(
        &self,
        table_id: &i64,
        order_id: &i64,
    ) -> Result<Table, (StatusCode, AxumErrors)> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //check if the table exists
        let filter = doc! {
            "table_id": table_id
        };
        //upsert order
        let update = doc! {
            "$pull":{
                "orders":{
                    "order_id":order_id
                }
            }
        };

        let options = mongodb::options::FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        match table_collection
            .find_one_and_update(filter, update, options)
            .await
        {
            Ok(opt_table) => match opt_table {
                Some(table) => Ok(table),
                None => Err((StatusCode::NOT_FOUND, AxumErrors::NotFound)),
            },
            Err(e) => {
                error!("Unexpected error occured while Deleting Order for Table in the Database. Error: {e}");
                Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError))
            }
        }
    }

    async fn get_order(
        &self,
        table_id: &i64,
        order_id: &i64,
    ) -> Result<Order, (StatusCode, AxumErrors)> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //check if the table and the order exist
        let filter = doc! {
            "table_id": table_id,
            "orders":{
                "$elemMatch":{
                    "order_id": order_id
                }
            }
        };

        match table_collection.find_one(filter, None).await {
            Ok(opt_table) => match opt_table {
                Some(table) => {
                    let order_vec: Vec<Order> = table
                        .orders
                        .into_iter()
                        .filter(|order| &order.order_id == order_id)
                        .collect();
                    Ok(order_vec[0].to_owned())
                }
                None => Err((StatusCode::NOT_FOUND, AxumErrors::NotFound)),
            },
            Err(e) => {
                error!("Unexpected error occured while searching for Order from Table in the Database. Error: {e}");
                Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError))
            }
        }
    }
    async fn list_table_orders(
        &self,
        table_id: &i64,
    ) -> Result<Vec<OrderResponse>, (StatusCode, AxumErrors)> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Table>("tables");

        //check if the table and the order exist
        let filter = doc! {
            "table_id": table_id,
        };

        match table_collection.find_one(filter, None).await {
            Ok(opt_table) => match opt_table {
                Some(table) => Ok(table
                    .orders
                    .into_iter()
                    .map(|order| order.into())
                    .collect::<Vec<OrderResponse>>()),
                None => Err((StatusCode::NOT_FOUND, AxumErrors::NotFound)),
            },
            Err(e) => {
                error!("Unexpected error occured while listing for Order from Table in the Database. Error: {e}");
                Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError))
            }
        }
    }

    //needs filters for the orders by completed orders, and
    async fn list_all_orders(
        &self,
        filters: &ListOrderFiltersRequest,
    ) -> Result<ListOrderResult, (StatusCode, AxumErrors)> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Document>("tables");

        let mut filter = match filters.table_ids.is_empty() {
            true => [doc! {
                "$match":{
                }
            }]
            .to_vec(),
            false => [doc! {
                "$match":{
                    "table_id":{"$in":filters.table_ids.clone()}
                }
            }]
            .to_vec(),
        };

        //now try to get the aggregate
        let aggregate_filter = list_all_orders_aggregate_helpers(filters.item_names.clone()).await;
        filter.extend(aggregate_filter);

        match table_collection.aggregate(filter, None).await {
            Ok(cursor) => {
                let (orders, failed_orders, dropped) =
                    collect_cursor::<Order, OrderResponse>(cursor)
                        .await
                        .get_results();
                Ok(ListOrderResult {
                    count: orders.len() as u64,
                    orders,
                    failed_orders: match failed_orders.len() {
                        0 => None,
                        _ => Some(failed_orders),
                    },
                    dropped: dropped,
                })
            }
            Err(e) => {
                error!("Unexpected error occured while listing all Orders from Table in the Database. Error: {e}");
                Err((StatusCode::INTERNAL_SERVER_ERROR, AxumErrors::DBError))
            }
        }
    }
}

pub async fn list_all_orders_aggregate_helpers(item_names: Vec<String>) -> Vec<Document> {
    let mut aggregate_doc = match item_names.is_empty() {
        true => [].to_vec(),
        false => [doc! {
            "$project":{
                "orders":{
                    "$filter":{
                        "input": "$orders",
                        "as": "order",
                        "cond": {
                                "$or":(
                                    item_names.into_iter().map(|item_name|{
                                        doc!{
                                            "$eq":[item_name, "$$order.item.item_name"]
                                        }
                                    }).collect::<Vec<Document>>()
                            )
                        }
                    }
                }
            }
        }]
        .to_vec(),
    };
    let projections = [
        doc! {
            "$unwind":"$orders"
        },
        doc! {
            "$project":{
                "orders":1
            }
        },
        doc! {
            "$replaceRoot" : {
                "newRoot" : "$orders"
            }
        },
    ]
    .to_vec();
    aggregate_doc.extend(projections);
    aggregate_doc
}

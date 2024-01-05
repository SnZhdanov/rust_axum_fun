use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::ReturnDocument;
use serde::{Deserialize, Serialize};

use crate::common::database;
use crate::common::database_helpers::collect_cursor;
use crate::common::models::pagination_schema::Pagination;
use crate::common::models::restaurant_schema::{Item, ItemResponse, Order, OrderResponse, Table};

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
    async fn get_table(&self, table_id: &i64) -> Result<Table, Box<dyn Error>>;
    async fn create_orders(
        &self,
        table_id: &i64,
        orders: Vec<Document>,
    ) -> Result<Table, Box<dyn Error>>;
    async fn get_items(&self, item_names: Vec<String>) -> Result<Vec<Item>, Box<dyn Error>>;
    async fn get_item(&self, item_name: String) -> Result<Option<Item>, Box<dyn Error>>;
    async fn delete_order(&self, table_id: &i64, order_id: &i64) -> Result<Table, Box<dyn Error>>;
    async fn get_order(&self, table_id: &i64, order_id: &i64) -> Result<Order, Box<dyn Error>>;
    async fn list_table_orders(&self, table_id: &i64)
        -> Result<Vec<OrderResponse>, Box<dyn Error>>;
    async fn list_all_orders(
        &self,
        pagination: &Pagination,
    ) -> Result<ListOrderResult, Box<dyn Error>>;
}

#[async_trait]
impl DBTableTrait for database::DB {
    async fn get_item(&self, item_name: String) -> Result<Option<Item>, Box<dyn Error>> {
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
                todo!()
            }
        }
    }

    async fn get_table(&self, table_id: &i64) -> Result<Table, Box<dyn Error>> {
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
                None => todo!(),
            },
            Err(e) => panic!("{e}"),
        }
    }

    async fn get_items(&self, item_names: Vec<String>) -> Result<Vec<Item>, Box<dyn Error>> {
        let item_collection = self
            .db
            .database("item_management")
            .collection::<Item>("items");

        let filter = doc! {
            "item_name":{
                "$in":item_names
            }
        };

        match item_collection.find(filter, None).await {
            Ok(cursor) => match cursor.try_collect::<Vec<Item>>().await {
                Ok(items) => Ok(items),
                Err(e) => panic!("Couldn't get the records! Error: {e}"),
            },
            Err(e) => {
                todo!()
            }
        }
    }

    async fn create_orders(
        &self,
        table_id: &i64,
        orders: Vec<Document>,
    ) -> Result<Table, Box<dyn Error>> {
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
                None => todo!(),
            },
            Err(e) => {
                panic!("{e}")
            }
        }
    }

    async fn delete_order(&self, table_id: &i64, order_id: &i64) -> Result<Table, Box<dyn Error>> {
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
                None => todo!(),
            },
            Err(e) => {
                panic!("{e}")
            }
        }
    }

    async fn get_order(&self, table_id: &i64, order_id: &i64) -> Result<Order, Box<dyn Error>> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Order>("tables");

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
            Ok(opt_order) => match opt_order {
                Some(order) => Ok(order),
                None => todo!(),
            },
            Err(e) => todo!(),
        }
    }
    async fn list_table_orders(
        &self,
        table_id: &i64,
    ) -> Result<Vec<OrderResponse>, Box<dyn Error>> {
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
                None => todo!(),
            },
            Err(e) => todo!(),
        }
    }

    //needs filters for the orders by completed orders, and
    async fn list_all_orders(
        &self,
        pagination: &Pagination,
    ) -> Result<ListOrderResult, Box<dyn Error>> {
        let table_collection = self
            .db
            .database("table_management")
            .collection::<Document>("tables");

        let find_options = mongodb::options::FindOptions::builder()
            .limit(pagination.limit)
            .skip(pagination.offset)
            .sort(None)
            .build();

        let filter = doc! {};

        let count_options = mongodb::options::CountOptions::builder().build();

        let count = match table_collection
            .count_documents(filter.clone(), Some(count_options))
            .await
        {
            Ok(count) => count,
            Err(e) => todo!(),
        };

        match table_collection.find(filter, find_options).await {
            Ok(cursor) => {
                let (orders, failed_orders, dropped) =
                    collect_cursor::<Order, OrderResponse>(cursor)
                        .await
                        .get_results();

                Ok(ListOrderResult {
                    orders,
                    failed_orders: match failed_orders.len() {
                        0 => None,
                        _ => Some(failed_orders),
                    },
                    count,
                    dropped: dropped,
                })
            }
            Err(e) => {
                // I want to eventually be able to handle errors gracefully here
                todo!()
            }
        }
    }
}

use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use futures::{StreamExt, TryStreamExt};
use mongodb::bson::{doc, Document};
use mongodb::options::ReturnDocument;
use serde::{Deserialize, Serialize};

use crate::common::database;
use crate::common::models::restaurant_schema::{Item, ItemResponse, Order, Table};

#[derive(Serialize)]
struct CreateOrderUpdateDoc {
    #[serde(rename = "$push")]
    push: Vec<Order>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Dat {
    ordered_time: DateTime<Utc>,
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
}

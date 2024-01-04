use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
//use futures::TryStreamExt;
use std::error::Error;

use crate::common::database;
use crate::common::database_helpers::collect_cursor;
use crate::common::models::pagination_schema::Pagination;
use crate::common::models::restaurant_schema::{Table, TableResponse};
use crate::common::models::sort_schema::SortDirectionBsonEnum;
use crate::common::models::sort_schema::SortDirectionEnum;

#[derive(Serialize, Deserialize)]
pub struct ListTablesResult {
    pub tables: Vec<TableResponse>,
    pub failed_tables: Option<Vec<String>>,
    pub count: u64,
    pub dropped: u64,
}

#[async_trait]
pub trait DBTableTrait {
    async fn create_table(&self, table: &Table) -> Result<Table, Box<dyn Error>>;
    async fn get_table(&self, table_id: i64) -> Result<Table, Box<dyn Error>>;
    async fn list_tables(
        &self,
        pagination: &Pagination,
    ) -> Result<ListTablesResult, Box<dyn Error>>;
    async fn delete_table(&self, table_id: i64) -> Result<TableResponse, Box<dyn Error>>;
}

#[async_trait]
impl DBTableTrait for database::DB {
    //this function is going to need session manager
    async fn create_table(&self, table: &Table) -> Result<Table, Box<dyn Error>> {
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
            Err(e) => panic!("{e}"),
        };

        // let table_as_bson = mongodb::bson::to_document(table).unwrap();

        match table_collection.insert_one(table, None).await {
            Ok(_) => Ok(table.clone().into()),
            Err(e) => {
                println!("{:?}", e);
                // I want to eventually be able to handle errors gracefully here
                todo!()
            }
        }
    }
    async fn get_table(&self, table_id: i64) -> Result<Table, Box<dyn Error>> {
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
                None => todo!(), //no record found!
            },
            Err(e) => {
                // I want to eventually be able to handle errors gracefully here
                todo!()
            }
        }
    }

    async fn list_tables(
        &self,
        pagination: &Pagination,
    ) -> Result<ListTablesResult, Box<dyn Error>> {
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
                let (tables, failed_tables, dropped) =
                    collect_cursor::<Table, TableResponse>(cursor)
                        .await
                        .get_results();

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
                // I want to eventually be able to handle errors gracefully here
                todo!()
            }
        }
    }
    async fn delete_table(&self, table_id: i64) -> Result<TableResponse, Box<dyn Error>> {
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
                None => todo!(), //no record found!
            },
            Err(e) => {
                // I want to eventually be able to handle errors gracefully here
                todo!()
            }
        };

        match table_collection.delete_one(filter, None).await {
            Ok(_) => Ok(table.into()),
            Err(e) => todo!(),
        }
    }
}
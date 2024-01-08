use super::models::restaurant_schema::Item;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use serde_json::Value;
use std::{error::Error, fs, time::Duration};

#[faux::create]
#[derive(Clone)]
pub struct DB {
    pub db: Client,
}

#[async_trait]
pub trait DBTrait {
    async fn init() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    async fn set_up_item_records(&self) -> Result<(), Box<dyn Error>>;
}

#[faux::methods]
#[async_trait]
impl DBTrait for DB {
    async fn init() -> Result<Self, Box<dyn Error>> {
        let connection_string: String =
            std::env::var("DATABASE_URL").expect("DATABASE_URL env var must be set!");
        let access_key: String = std::env::var("MONGO_INITDB_ROOT_USERNAME")
            .unwrap_or("MONGO_INITDB_ROOT_USERNAME env var must be set!".to_string());
        let secret_key: String = std::env::var("MONGO_INITDB_ROOT_PASSWORD")
            .unwrap_or("MONGO_INITDB_ROOT_PASSWORD env var must be set!".to_string());

        let connection_string = connection_string
            .replace("<MONGO_INITDB_ROOT_USERNAME>", access_key.as_str())
            .replace(
                "<MONGO_INITDB_ROOT_PASSWORD>",
                utf8_percent_encode(secret_key.as_str(), NON_ALPHANUMERIC)
                    .to_string()
                    .as_str(),
            );
            
        match ClientOptions::parse(connection_string).await {
            Ok(mut client_options) => {
                client_options.app_name = Some("MongoDB Client".to_string());
                client_options.max_pool_size = Some(10);
                client_options.min_pool_size = Some(1);
                client_options.connect_timeout = Some(Duration::from_secs(5));
                client_options.max_idle_time = Some(Duration::from_secs(30));

                Ok(Self {
                    db: Client::with_options(client_options)?,
                })
            }
            Err(e) => Err(format!("Could not connect to the DB: {e}").into()),
        }
    }

    async fn set_up_item_records(&self) -> Result<(), Box<dyn Error>> {
        let item_collection = self
            .db
            .database("item_management")
            .collection::<Item>("items");

        match item_collection.find(doc! {}, None).await {
            Ok(cursor) => {
                let items: Vec<Item> = match cursor.try_collect().await {
                    Ok(items) => items,
                    Err(e) => panic!("Couldn't get the records! Error: {e}"),
                };

                if items.is_empty() {
                    ()
                } else {
                    return Ok(());
                }
            }
            Err(e) => {
                panic!("unexpected error finding the records  when populating the DB: {e}")
                //drop all the records
            }
        };

        let file_path: String =
            std::env::var("FILE_PATH").expect("FILE_PATH env var must be set!");

        let file = fs::File::open(file_path)
            .expect("File for pre-loading the DB not found!");

        let json: Value = serde_json::from_reader(file)
            .expect("Was unable to read the file for pre-loading the DB!");

        let records_arr: Vec<Item> = match json.get("records") {
            Some(records) => serde_json::from_value(records.clone())
                .expect("unable to parse the item records in the file for pre-loading the DB!"),
            None => panic!(
                "Was unable to find the record's key. Check the file for pre-loading the DB!"
            ),
        };

        match item_collection.insert_many(records_arr, None).await {
            Ok(_) => Ok(()),
            Err(e) => panic!(
                "Unexpected error while inserting many records into the Item Database! Error: {e}"
            ),
        }
    }
}

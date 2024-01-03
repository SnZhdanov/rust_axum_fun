use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};
use std::{error::Error, time::Duration};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

#[derive(Clone)]
pub struct DB {
    pub db: Client,
}

#[async_trait]
pub trait DBTrait {
    async fn init() -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

#[async_trait]
impl DBTrait for DB {
    async fn init() -> Result<Self, Box<dyn Error>> {
        let connection_string: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let access_key: String = std::env::var("MONGO_INITDB_ROOT_USERNAME").unwrap_or("".to_string());
        let secret_key: String = std::env::var("MONGO_INITDB_ROOT_PASSWORD").unwrap_or("".to_string());

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
                client_options.connect_timeout = Some(Duration::from_secs(30));
                client_options.max_idle_time = Some(Duration::from_secs(120));
            
                Ok(Self {
                    db: Client::with_options(client_options)?,
                })
            }
            Err(e) => Err(format!("Could not connect to the DB: {e}").into()),
        }
    }
}
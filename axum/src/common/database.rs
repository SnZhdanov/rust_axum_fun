use async_trait::async_trait;
use mongodb::{options::ClientOptions, Client};
use std::error::Error;
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
        let connection_string: String = std::env::var("DB_URL").expect("DB_URL must be set");
        let access_key: String = std::env::var("DB_USER_NAME").unwrap_or("".to_string());
        let secret_key: String = std::env::var("DB_USER_PASSWORD").unwrap_or("".to_string());

        let connection_string = connection_string
            .replace("<DB_USER_NAME>", access_key.as_str())
            .replace(
                "<DB_USER_PASSWORD>",
                utf8_percent_encode(secret_key.as_str(), NON_ALPHANUMERIC)
                    .to_string()
                    .as_str(),
            );

        match ClientOptions::parse(connection_string).await {
            Ok(mut client_options) => {
                client_options.app_name = Some("MongoDB Client".to_string());
                Ok(Self {
                    db: Client::with_options(client_options)?,
                })
            }
            Err(e) => Err(format!("Could not connect to the DB: {e}").into()),
        }
    }
}
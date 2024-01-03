use crate::common::database::DB;
use async_trait::async_trait;
//use futures::TryStreamExt;
use std::error::Error;

use super::models::restaurant_schema::{Table, Order};

#[async_trait]
pub trait DBTableTrait{
    async fn create_table(&self, table: Table)-> Result<(), Box<dyn Error>>;
    async fn get_table(&self) -> Result<(), Box<dyn Error>>;
    async fn list_tables(&self) -> Result<(), Box<dyn Error>>;
    async fn delete_table(&self) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
pub trait DBOrderTrait{
    async fn create_order(&self, new_order: Order)-> Result<(), Box<dyn Error>>;
    async fn get_order(&self) -> Result<(), Box<dyn Error>>;
    async fn list_orders(&self) -> Result<(), Box<dyn Error>>;
    async fn delete_order(&self) -> Result<(), Box<dyn Error>>;
}
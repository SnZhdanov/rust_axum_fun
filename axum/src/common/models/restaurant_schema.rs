use serde::{Serialize, Deserialize};
use chrono::{prelude::*, DateTime};
use mongodb::bson::serde_helpers::{chrono_datetime_as_bson_datetime, hex_string_as_object_id};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table{
    #[serde(rename = "_id", with="hex_string_as_object_id")]
    id: String,     //kept private to prevent user interaction
    pub table_id: u64,
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order{
    pub order_id: u64,
    pub table_id: u64,
    pub item: Item,
    pub cook_status: CookStatus
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item{
    pub item_name: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub cook_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CookStatus{
    InProgress,
    Done
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableResponse{
    pub table_id: u64,
    pub orders: Vec<OrderResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResponse{
    pub order_id: u64,
    pub table_id: u64,
    pub item: ItemResponse,
    pub cook_status: CookStatus
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemResponse{
    pub item_name: String,
    pub cook_time: DateTime<Utc>,
}

impl From<Table> for TableResponse{
    fn from(table: Table) -> Self{
        let Table { 
            table_id, 
            orders,
            ..
        } = table;
        Self { 
            table_id: table_id, 
            orders: orders.into_iter().map(|order| order.into()).collect()
        }
    }
}

impl From<Order> for OrderResponse{
    fn from(order: Order) -> Self{
        let Order { 
            order_id, 
            table_id, 
            item, 
            cook_status,
            .. 
        } = order;
        Self { 
            order_id, 
            table_id, 
            item: item.into(), 
            cook_status
        }
    }
}

impl From<Item> for ItemResponse{
    fn from(item: Item) -> Self{
        let Item { 
            item_name, 
            cook_time 
        } = item;
        Self { 
            item_name, 
            cook_time
        }
    }
}
use std::str::FromStr;

use chrono::{prelude::*, DateTime, Duration};
use mongodb::bson::serde_helpers::{chrono_datetime_as_bson_datetime, hex_string_as_object_id};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    #[serde(rename = "_id", with = "hex_string_as_object_id")]
    pub id: String, //kept private to prevent user interaction
    pub table_id: i64,
    pub orders: Vec<Order>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub order_id: i64,
    pub table_id: i64,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub ordered_time: DateTime<Utc>,
    pub cook_status: CookStatus,
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub item_name: String,
    pub cook_time: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CookStatus {
    InProgress,
    Done,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TableResponse {
    pub table_id: i64,
    pub orders: Vec<OrderResponse>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderResponse {
    pub order_id: i64,
    pub table_id: i64,
    pub ordered_time: DateTime<Utc>,
    pub cook_status: CookStatus,
    pub item: ItemResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemResponse {
    pub item_name: String,
    pub cook_time: i64,
}

impl From<Table> for TableResponse {
    fn from(table: Table) -> Self {
        let Table {
            table_id, orders, ..
        } = table;
        Self {
            table_id: table_id,
            orders: orders.into_iter().map(|order| order.into()).collect(),
        }
    }
}

impl From<Order> for OrderResponse {
    fn from(order: Order) -> Self {
        let Order {
            order_id,
            table_id,
            ordered_time,
            item,
            ..
        } = order;
        Self {
            order_id,
            table_id,
            ordered_time: ordered_time.clone(),
            cook_status: {
                let current_time = Utc::now();
                let time_cooked = ordered_time + Duration::seconds(item.cook_time);
                if current_time >= time_cooked {
                    CookStatus::Done
                } else {
                    CookStatus::InProgress
                }
            },
            item: item.into(),
        }
    }
}

impl From<Item> for ItemResponse {
    fn from(item: Item) -> Self {
        let Item {
            item_name,
            cook_time,
        } = item;
        Self {
            item_name,
            cook_time,
        }
    }
}

impl ToString for CookStatus {
    fn to_string(&self) -> String {
        match self {
            CookStatus::InProgress => "InProgress".into(),
            CookStatus::Done => "Done".into(),
        }
    }
}

impl FromStr for CookStatus {
    type Err = ();
    fn from_str(input: &str) -> Result<CookStatus, Self::Err> {
        match input {
            "InProgress" => Ok(CookStatus::InProgress),
            "Done" => Ok(CookStatus::Done),
            _ => Err(()),
        }
    }
}

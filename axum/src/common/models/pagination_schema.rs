use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Pagination{
    pub offset: u64,
    pub limit: u64
}
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pagination {
    #[serde(default = "default_offset")]
    pub offset: u64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_offset() -> u64 {
    0
}

fn default_limit() -> i64 {
    10
}

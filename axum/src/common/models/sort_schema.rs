use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Clone)]
pub enum SortDirectionEnum {
    Ascending,
    Descending,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(i8)]
pub enum SortDirectionBsonEnum {
    Ascending = 1,
    Descending = -1,
}

impl ToString for SortDirectionEnum {
    fn to_string(&self) -> String {
        match self {
            SortDirectionEnum::Ascending => "Ascending".into(),
            SortDirectionEnum::Descending => "Descending".into(),
        }
    }
}

impl Into<i8> for SortDirectionBsonEnum {
    fn into(self) -> i8 {
        match self {
            SortDirectionBsonEnum::Ascending => 1,
            SortDirectionBsonEnum::Descending => -1,
        }
    }
}

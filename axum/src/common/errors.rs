use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub error: AxumErrorResponse,
}

impl ErrorResponse {
    pub fn to_axum_error(self) -> (StatusCode, Json<AxumErrorResponse>) {
        (self.status_code, Json(self.error))
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Path not found!")
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AxumErrors {
    NotFound,
    SerializationError,
    DeserializationError,
    DBError,
    BsonSerializeError,
    BsonDeserializeError,
}

#[derive(Serialize, Deserialize)]
pub struct AxumErrorResponse {
    pub error_type: String,
    pub error_message: String,
}

impl From<AxumErrors> for AxumErrorResponse {
    fn from(axum_errors: AxumErrors) -> Self {
        match axum_errors {
            AxumErrors::NotFound => Self {
                error_type: "NotFound".to_string(),
                error_message: "Document Was Not Found!".to_string(),
            },
            AxumErrors::SerializationError => Self {
                error_type: "SerializationError".to_string(),
                error_message: "Serde Serialization Error Occurred!".to_string(),
            },
            AxumErrors::DeserializationError => Self {
                error_type: "DeserializationError".to_string(),
                error_message: "Serde Deserialization Error Occurred!".to_string(),
            },
            AxumErrors::DBError => Self {
                error_type: "DBError".to_string(),
                error_message: "Unexpected Error response from MongoDB!".to_string(),
            },
            AxumErrors::BsonSerializeError => Self {
                error_type: "BsonSerializeError".to_string(),
                error_message: "Unexpected error from serializing struct into Bson Document!"
                    .to_string(),
            },
            AxumErrors::BsonDeserializeError => Self {
                error_type: "BsonDeserializeError".to_string(),
                error_message: "Unexpected error from deserializing struct into Bson Document!"
                    .to_string(),
            },
        }
    }
}

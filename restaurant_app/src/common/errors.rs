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

impl ToString for AxumErrors {
    fn to_string(&self) -> String {
        match self {
            AxumErrors::NotFound => "NotFound".into(),
            AxumErrors::SerializationError => "SerializationError".into(),
            AxumErrors::DeserializationError => "DeserializationError".into(),
            AxumErrors::DBError => "DBError".into(),
            AxumErrors::BsonSerializeError => "BsonSerializeError".into(),
            AxumErrors::BsonDeserializeError => "BsonDeserializeError".into(),
        }
    }
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
                error_type: AxumErrors::NotFound.to_string(),
                error_message: "Document Was Not Found!".to_string(),
            },
            AxumErrors::SerializationError => Self {
                error_type: AxumErrors::SerializationError.to_string(),
                error_message: "Serde Serialization Error Occurred!".to_string(),
            },
            AxumErrors::DeserializationError => Self {
                error_type: AxumErrors::DeserializationError.to_string(),
                error_message: "Serde Deserialization Error Occurred!".to_string(),
            },
            AxumErrors::DBError => Self {
                error_type: AxumErrors::DBError.to_string(),
                error_message: "Unexpected Error response from MongoDB!".to_string(),
            },
            AxumErrors::BsonSerializeError => Self {
                error_type: AxumErrors::BsonSerializeError.to_string(),
                error_message: "Unexpected error from serializing struct into Bson Document!"
                    .to_string(),
            },
            AxumErrors::BsonDeserializeError => Self {
                error_type: AxumErrors::BsonDeserializeError.to_string(),
                error_message: "Unexpected error from deserializing struct into Bson Document!"
                    .to_string(),
            },
        }
    }
}

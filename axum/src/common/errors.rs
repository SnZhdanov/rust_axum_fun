use axum::{http::StatusCode, response::IntoResponse};
pub async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "Path does not exist or data was not found!",
    )
}

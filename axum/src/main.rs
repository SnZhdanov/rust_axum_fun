use axum::{Router, routing::{get, post, delete}, extract::Query};
use common::models::pagination_schema::Pagination;


mod common;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/table", post(|| async { "Hello, World!" }))
        .route("/table/:table_id/order", post(|| async {"Hello, World!"}))
        .route("/table/list", get(|| async { "Hello, World!" }))
        .route("/table/:table_id", get(|| async { "Hello, World!" }))
        .route("/table/:table_id/order/list", get(|| async { "Hello, World!" }))
        .route("/table/:table_id/order/:order_id", get(|| async { "Hello, World!" }))
        .route("/table/:table_id", delete(|| async { "Hello, World!" }))
        .route("/table/:table_id/order/:order_id", delete(|| async { "Hello, World!" }))
        .route("/item/list", get(|| async{ "Hello, World!"}));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_many_items(Query(pagination_params): Query<Pagination>) -> String {

    let pagination = Pagination {
        offset: pagination_params.offset,
        limit: pagination_params.limit,
    };

    let message = format!("offset={} & limit={}", pagination.offset, pagination.limit);
    println!("{}", &message);

    message
}
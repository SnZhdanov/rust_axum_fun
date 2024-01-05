mod common;
mod handlers;
use std::sync::{atomic::AtomicUsize, Arc};

use axum::{
    extract::Query,
    routing::{delete, get, post},
    Router,
};
use common::database::{DBTrait, DB};
use common::errors::handler_404;
use common::models::pagination_schema::Pagination;
use handlers::{order_handler, table_handler};

use tokio::sync::Mutex;

struct AppState {
    // We require unique table ids
    tables: Mutex<i64>,
    orders: Mutex<i64>,
    // db pool
    db: DB,
}

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    //initializing the tracer
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();
    //initialize the db connection
    let db = match DB::init().await {
        Ok(db) => db,
        Err(e) => {
            panic!("Was unable to initialize the db! {e}");
        }
    };

    //pre-load the database with items
    match db.set_up_item_records().await {
        Ok(_) => (),
        Err(e) => {
            todo!()
        }
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to the Restaurant!" }))
        .route("/table", post(table_handler::table::create_table))
        .route("/table", get(table_handler::table::list_table))
        .route("/table/order", get(order_handler::order::list_all_orders))
        .route("/table/:table_id", get(table_handler::table::get_table))
        .route(
            "/table/:table_id",
            delete(table_handler::table::delete_table),
        )
        .route(
            "/table/:table_id/order",
            post(order_handler::order::create_order),
        )
        .route(
            "/table/:table_id/order/:order_id",
            get(order_handler::order::get_order),
        )
        .route(
            "/table/:table_id/order/:order_id",
            delete(order_handler::order::delete_order),
        )
        .route("/item", get(|| async { "Hello, World!" }));

    let app_state = Arc::new(AppState {
        db,
        tables: Mutex::new(0),
        orders: Mutex::new(0),
    });
    let app = app.fallback(handler_404).with_state(app_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests;

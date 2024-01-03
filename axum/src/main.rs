mod common;
mod handlers;
use std::sync::{atomic::AtomicUsize, Arc};

use axum::{Router, routing::{get, post, delete}, extract::Query};
use common::models::pagination_schema::Pagination;
use common::errors::handler_404;
use common::database::{DB, DBTrait};
use handlers::table;
use tokio::sync::Mutex;


struct AppState {
    // We require unique table ids
    tables: Mutex<u64>,
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
    let db = match DB::init().await{
        Ok(db) => db,
        Err(e) =>{
            panic!("Was unable to initialize the db! {e}");
        }
    };

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Welcome to the Restaurant!" }))
        .route("/table", post(table::create_table))
        .route("/table/:table_id/order", post(|| async {"Hello, World!"}))
        .route("/table/list", get(|| async { "Hello, World!" }))
        .route("/table/:table_id", get(|| async { "Hello, World!" }))
        .route("/table/:table_id/order/list", get(|| async { "Hello, World!" }))
        .route("/table/:table_id/order/:order_id", get(|| async { "Hello, World!" }))
        .route("/table/:table_id", delete(|| async { "Hello, World!" }))
        .route("/table/:table_id/order/:order_id", delete(|| async { "Hello, World!" }))
        .route("/item/list", get(|| async{ "Hello, World!"}));

    let app_state = Arc::new(AppState{
        db,
        tables: Mutex::new(0)
    });
    let app = app.fallback(handler_404).with_state(app_state);


    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// async fn get_many_items(Query(pagination_params): Query<Pagination>) -> String {

//     let pagination = Pagination {
//         offset: pagination_params.offset,
//         limit: pagination_params.limit,
//     };

//     let message = format!("offset={} & limit={}", pagination.offset, pagination.limit);
//     println!("{}", &message);

//     message
// }


#[cfg(test)]
mod tests;
#[cfg(test)]
pub mod order_unit_tests {

    use axum::{extract::State, http::StatusCode};
    use axum_extra::extract::Query as ExtraQuery;
    use std::sync::Arc;

    use tokio::sync::Mutex;

    use crate::{
        common::{
            database::DB,
            errors::{AxumErrors, ErrorResponse},
            models::{pagination_schema::Pagination, restaurant_schema::ItemResponse},
        },
        handlers::item_handler::{
            item::{list_items, ListItemsRequest},
            item_db::ListItemResults,
        },
        AppState,
    };

    #[tokio::test]
    pub async fn successful_list_items() {
        let mut mock_db = DB::faux();

        let items = [ItemResponse {
            item_name: "Burger".to_string(),
            cook_time: 5,
        }]
        .to_vec();

        let list_item_results = ListItemResults {
            items,
            failed_items: None,
            count: 1,
            dropped: 0,
        };

        faux::when!(mock_db.list_items).then(move |_| Ok(list_item_results.to_owned()));
        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });

        let state = State(app_state);
        let pagination = ExtraQuery(Pagination {
            offset: 0,
            limit: 10,
        });
        let query = ExtraQuery(ListItemsRequest {
            item_names: ["Burger".to_string()].to_vec(),
        });
        match list_items(state, pagination, query).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_list_items_db_error() {
        let mut mock_db = DB::faux();
        faux::when!(mock_db.list_items).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::DBError.into(),
            })
        });
        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });

        let state = State(app_state);
        let pagination = ExtraQuery(Pagination {
            offset: 0,
            limit: 10,
        });
        let query = ExtraQuery(ListItemsRequest {
            item_names: ["Burger".to_string()].to_vec(),
        });
        match list_items(state, pagination, query).await {
            Ok(_) => panic!("not supposed to succeed"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_list_items_b_deserialization_error() {
        let mut mock_db = DB::faux();
        faux::when!(mock_db.list_items).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::BsonDeserializeError.into(),
            })
        });
        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });

        let state = State(app_state);
        let pagination = ExtraQuery(Pagination {
            offset: 0,
            limit: 10,
        });
        let query = ExtraQuery(ListItemsRequest {
            item_names: ["Burger".to_string()].to_vec(),
        });
        match list_items(state, pagination, query).await {
            Ok(_) => panic!("not supposed to succeed"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::BsonDeserializeError.to_string());
            }
        }
    }
}

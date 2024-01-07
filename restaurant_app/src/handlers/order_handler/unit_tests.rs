#[cfg(test)]
pub mod order_unit_tests {

    use std::sync::Arc;

    use axum::{
        extract::{Path, Query, State},
        http::StatusCode,
        Json,
    };
    use axum_extra::extract::Query as ExtraQuery;
    use chrono::Utc;
    use mongodb::bson::oid::ObjectId;
    use tokio::sync::Mutex;

    use crate::{
        common::{
            database::DB,
            errors::{AxumErrors, ErrorResponse},
            models::{
                pagination_schema::Pagination,
                restaurant_schema::{CookStatus, Item, ItemResponse, Order, OrderResponse, Table},
            },
        },
        handlers::order_handler::{
            order::{
                create_order, delete_order, get_order, list_all_orders, CreateOrdersRequest,
                ListOrderFiltersRequest,
            },
            order_db::ListOrderResult,
        },
        AppState,
    };

    ////////////////////////
    //                   //
    //  CREATE_TABLE    //
    //                 //
    ////////////////////

    #[tokio::test]
    pub async fn successful_create_order() {
        let mut mock_db = DB::faux();
        let mut table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };

        let item = Item {
            item_name: "Burger".to_string(),
            cook_time: 5,
        };

        let order = Order {
            order_id: 1,
            table_id: 1,
            ordered_time: Utc::now(),
            cook_status: CookStatus::InProgress,
            item: item.clone(),
        };

        let table_get = table.clone();

        table.orders = [order.clone()].to_vec();

        let table_create = table.clone();

        faux::when!(mock_db.get_table_order).then(move |_| Ok(table_get.to_owned()));

        faux::when!(mock_db.get_item).then(move |_| Ok(Some(item.to_owned())));

        faux::when!(mock_db.create_orders).then(move |_| Ok(table_create.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path(1);
        let json_body = Json(CreateOrdersRequest {
            orders: ["Burger".to_string()].to_vec(),
        });
        match create_order(state, path, json_body).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::CREATED);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_create_order_get_table_not_found_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.get_table_order).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: AxumErrors::NotFound.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path(1);
        let json_body = Json(CreateOrdersRequest {
            orders: ["Burger".to_string()].to_vec(),
        });
        match create_order(state, path, json_body).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::NotFound.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_create_order_get_table_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.get_table_order).then(move |_| {
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
        let path = Path(1);
        let json_body = Json(CreateOrdersRequest {
            orders: ["Burger".to_string()].to_vec(),
        });
        match create_order(state, path, json_body).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_create_order_get_item_db_error() {
        let mut mock_db = DB::faux();
        let mut table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };

        let item = Item {
            item_name: "Burger".to_string(),
            cook_time: 5,
        };

        let order = Order {
            order_id: 1,
            table_id: 1,
            ordered_time: Utc::now(),
            cook_status: CookStatus::InProgress,
            item: item.clone(),
        };

        let table_get = table.clone();

        table.orders = [order.clone()].to_vec();

        faux::when!(mock_db.get_table_order).then(move |_| Ok(table_get.to_owned()));

        faux::when!(mock_db.get_item).then(move |_| {
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
        let path = Path(1);
        let json_body = Json(CreateOrdersRequest {
            orders: ["Burger".to_string()].to_vec(),
        });
        match create_order(state, path, json_body).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_create_order_create_orders_db_error() {
        let mut mock_db = DB::faux();
        let table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };

        let item = Item {
            item_name: "Burger".to_string(),
            cook_time: 5,
        };

        faux::when!(mock_db.get_table_order).then(move |_| Ok(table.to_owned()));

        faux::when!(mock_db.get_item).then(move |_| Ok(Some(item.to_owned())));

        faux::when!(mock_db.create_orders).then(move |_| {
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
        let path = Path(1);
        let json_body = Json(CreateOrdersRequest {
            orders: ["Burger".to_string()].to_vec(),
        });
        match create_order(state, path, json_body).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_create_order_create_orders_not_found_error() {
        let mut mock_db = DB::faux();
        let table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };

        let item = Item {
            item_name: "Burger".to_string(),
            cook_time: 5,
        };

        faux::when!(mock_db.get_table_order).then(move |_| Ok(table.to_owned()));

        faux::when!(mock_db.get_item).then(move |_| Ok(Some(item.to_owned())));

        faux::when!(mock_db.create_orders).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: AxumErrors::NotFound.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path(1);
        let json_body = Json(CreateOrdersRequest {
            orders: ["Burger".to_string()].to_vec(),
        });
        match create_order(state, path, json_body).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::NotFound.to_string());
            }
        }
    }

    //////////////////////
    //                  //
    //  LIST_ORDERS    //
    //                 //
    ////////////////////

    #[tokio::test]
    pub async fn successful_list_all_orders() {
        let mut mock_db = DB::faux();
        let orders = [OrderResponse {
            order_id: 1,
            table_id: 1,
            ordered_time: Utc::now(),
            cook_status: CookStatus::InProgress,
            item: ItemResponse {
                item_name: "Hamburger".to_string(),
                cook_time: 5,
            },
        }]
        .to_vec();

        let list_order_result = ListOrderResult {
            count: 1,
            dropped: 0,
            orders,
            failed_orders: None,
        };

        faux::when!(mock_db.list_all_orders).then(move |_| Ok(list_order_result.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let pagination = Query(Pagination {
            offset: 0,
            limit: 10,
        });
        let filters: ExtraQuery<ListOrderFiltersRequest> = ExtraQuery(ListOrderFiltersRequest {
            table_ids: [1].to_vec(),
            item_names: ["Hamburger".to_string()].to_vec(),
            cook_status: Some(CookStatus::InProgress),
        });
        match list_all_orders(state, pagination, filters).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_list_all_orders_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.list_all_orders).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::DBError.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let pagination = Query(Pagination {
            offset: 0,
            limit: 10,
        });
        let filters: ExtraQuery<ListOrderFiltersRequest> = ExtraQuery(ListOrderFiltersRequest {
            table_ids: [1].to_vec(),
            item_names: ["Hamburger".to_string()].to_vec(),
            cook_status: Some(CookStatus::InProgress),
        });
        match list_all_orders(state, pagination, filters).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_list_all_orders_b_deserialize_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.list_all_orders).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::BsonDeserializeError.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let pagination = Query(Pagination {
            offset: 0,
            limit: 10,
        });
        let filters: ExtraQuery<ListOrderFiltersRequest> = ExtraQuery(ListOrderFiltersRequest {
            table_ids: [1].to_vec(),
            item_names: ["Hamburger".to_string()].to_vec(),
            cook_status: Some(CookStatus::InProgress),
        });
        match list_all_orders(state, pagination, filters).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::BsonDeserializeError.to_string());
            }
        }
    }

    ////////////////////////
    //                   //
    //  DELETE_TABLE    //
    //                 //
    ////////////////////

    #[tokio::test]
    pub async fn successful_delete_order() {
        let mut mock_db = DB::faux();
        let mut table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };

        let item = Item {
            item_name: "Burger".to_string(),
            cook_time: 5,
        };

        let order = Order {
            order_id: 1,
            table_id: 1,
            ordered_time: Utc::now(),
            cook_status: CookStatus::InProgress,
            item: item.clone(),
        };

        table.orders = [order.clone()].to_vec();

        faux::when!(mock_db.delete_order).then(move |_| Ok(table.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path((1, 1));
        match delete_order(state, path).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_delete_order_not_found() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.delete_order).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: AxumErrors::NotFound.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(1),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path((1, 1));
        match delete_order(state, path).await {
            Ok(_) => panic!("not supposed to succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::NotFound.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_delete_order_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.delete_order).then(move |_| {
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
        let path = Path((1, 1));
        match delete_order(state, path).await {
            Ok(_) => panic!("not supposed to succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    ////////////////////////
    //                   //
    //  GET_ORDER       //
    //                 //
    ////////////////////

    #[tokio::test]
    pub async fn successful_get_order() {
        let mut mock_db = DB::faux();
        let order = Order {
            order_id: 1,
            table_id: 1,
            ordered_time: Utc::now(),
            cook_status: CookStatus::InProgress,
            item: Item {
                item_name: "Hamburger".to_string(),
                cook_time: 5,
            },
        };

        faux::when!(mock_db.get_order).then(move |_| Ok(order.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path((1, 1));
        match get_order(state, path).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_get_order_not_found() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.get_order).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: AxumErrors::NotFound.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path((1, 1));
        match get_order(state, path).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::NotFound.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_get_order_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.get_order).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::DBError.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path((1, 1));
        match get_order(state, path).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }
}

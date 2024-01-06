#[cfg(test)]
pub mod table_unit_tests {

    use std::sync::Arc;

    use axum::{
        extract::{Path, Query, State},
        http::StatusCode,
    };
    use axum_extra::extract::Query as ExtraQuery;
    use mongodb::bson::oid::ObjectId;
    use tokio::sync::Mutex;

    use crate::{
        common::{
            database::DB,
            errors::{AxumErrors, ErrorResponse},
            models::{
                pagination_schema::Pagination,
                restaurant_schema::{Table, TableResponse},
            },
        },
        handlers::table_handler::{
            table::{create_table, delete_table, get_table, list_table, ListTableFiltersRequest},
            table_db::ListTablesResult,
        },
        AppState,
    };

    ////////////////////
    //  CREATE_TABLE  //
    ////////////////////

    #[tokio::test]
    pub async fn successful_create() {
        let mut mock_db = DB::faux();
        let table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };
        faux::when!(mock_db.create_table).then(move |_| Ok(table.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(0),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);

        match create_table(state).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::CREATED);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_create_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.create_table).then(move |_| {
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

        match create_table(state).await {
            Ok(_) => panic!("supposed to fail"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    ////////////////////
    //  LIST_TABLE    //
    ////////////////////

    #[tokio::test]
    pub async fn successful_list_table() {
        let mut mock_db = DB::faux();
        let table = [TableResponse {
            table_id: 1,
            orders: [].to_vec(),
        }]
        .to_vec();

        let list_table_result = ListTablesResult {
            tables: table,
            failed_tables: None,
            count: 1,
            dropped: 0,
        };

        faux::when!(mock_db.list_tables).then(move |_| Ok(list_table_result.to_owned()));

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
        let query: ExtraQuery<ListTableFiltersRequest> = ExtraQuery(ListTableFiltersRequest {
            table_id: None,
            order_id: None,
            item_name: None,
            item_names: [].to_vec(),
        });
        match list_table(state, pagination, query).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_list_table_bson() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.list_tables).then(move |_| {
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
        let query: ExtraQuery<ListTableFiltersRequest> = ExtraQuery(ListTableFiltersRequest {
            table_id: None,
            order_id: None,
            item_name: None,
            item_names: [].to_vec(),
        });
        match list_table(state, pagination, query).await {
            Ok(_) => panic!("supposed to fail"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_list_table_bson_d_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.list_tables).then(move |_| {
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
        let query: ExtraQuery<ListTableFiltersRequest> = ExtraQuery(ListTableFiltersRequest {
            table_id: None,
            order_id: None,
            item_name: None,
            item_names: [].to_vec(),
        });
        match list_table(state, pagination, query).await {
            Ok(_) => panic!("supposed to fail"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::BsonDeserializeError.to_string());
            }
        }
    }

    ////////////////////
    //  GET_TABLE     //
    ////////////////////

    #[tokio::test]
    pub async fn successful_get_table() {
        let mut mock_db = DB::faux();
        let table = Table {
            id: ObjectId::new().to_hex(),
            table_id: 1,
            orders: [].to_vec(),
        };

        faux::when!(mock_db.get_table).then(move |_| Ok(table.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path(1);
        match get_table(state, path).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_get_table_not_found() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.get_table).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: AxumErrors::NotFound.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path(1);
        match get_table(state, path).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::NotFound.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_get_table_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.get_table).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::DBError.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });
        let state = State(app_state);
        let path = Path(1);
        match get_table(state, path).await {
            Ok(_) => panic!("shouldn't succeed!"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }

    ////////////////////////
    //  DELETE_TABLE     //
    //////////////////////

    #[tokio::test]
    pub async fn successful_delete_table() {
        let mut mock_db = DB::faux();
        let table = TableResponse {
            table_id: 1,
            orders: [].to_vec(),
        };

        faux::when!(mock_db.delete_table).then(move |_| Ok(table.to_owned()));

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });

        let state = State(app_state);
        let path = Path(1);
        match delete_table(state, path).await {
            Ok(resp) => {
                assert_eq!(resp.0, StatusCode::OK);
            }
            Err(e) => panic!("error! {:?}", e.1.error_type),
        }
    }

    #[tokio::test]
    pub async fn failed_delete_table_not_found() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.delete_table).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::NOT_FOUND,
                error: AxumErrors::NotFound.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });

        let state = State(app_state);
        let path = Path(1);
        match delete_table(state, path).await {
            Ok(_) => panic!("not supposed to succeed"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::NotFound.to_string());
            }
        }
    }

    #[tokio::test]
    pub async fn failed_delete_table_db_error() {
        let mut mock_db = DB::faux();

        faux::when!(mock_db.delete_table).then(move |_| {
            Err(ErrorResponse {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                error: AxumErrors::DBError.into(),
            })
        });

        let app_state = Arc::new(AppState {
            tables: Mutex::new(1),
            orders: Mutex::new(0),
            db: mock_db.clone(),
        });

        let state = State(app_state);
        let path = Path(1);
        match delete_table(state, path).await {
            Ok(_) => panic!("not supposed to succeed"),
            Err(e) => {
                assert_eq!(e.1.error_type, AxumErrors::DBError.to_string());
            }
        }
    }
}

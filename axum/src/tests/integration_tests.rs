#[cfg(test)]
pub mod integration_tests {
    use futures::future::join_all;
    use serde::Serialize;

    use crate::{
        common::models::restaurant_schema::CookStatus,
        handlers::{
            order_handler::order::{CreateOrdersRequest, ListOrdersResponse, ReturnTableResponse},
            table_handler::table::PostTableResponse,
        },
    };
    use tokio::time::{sleep, Duration};

    #[derive(Serialize)]
    struct CreateTableOrder {
        orders: Vec<String>,
    }

    #[derive(Serialize)]
    struct ListOrderQuery {
        cook_status: CookStatus,
    }

    async fn create_table_call(orders: Vec<String>) -> PostTableResponse {
        let client = reqwest::Client::new();
        let orders = CreateTableOrder { orders };
        let create_call = client
            .post("http://127.0.0.1:8080/table")
            .header("Content-Type", "application/json")
            .json(&orders)
            .send()
            .await
            .unwrap()
            .json::<PostTableResponse>()
            .await
            .unwrap();
        return create_call;
    }

    async fn list_all_orders(list_order_query: Option<ListOrderQuery>) -> ListOrdersResponse {
        let client = reqwest::Client::new();

        let list_call = client
            .get("http://127.0.0.1:8080/table/order")
            .header("Content-Type", "application/json");

        match list_order_query {
            Some(list_order_query) => list_call
                .query(&list_order_query)
                .send()
                .await
                .unwrap()
                .json::<ListOrdersResponse>()
                .await
                .unwrap(),
            None => list_call
                .send()
                .await
                .unwrap()
                .json::<ListOrdersResponse>()
                .await
                .unwrap(),
        }
    }

    async fn create_order_call(table_id: i64, orders: Vec<String>) -> ReturnTableResponse {
        let client = reqwest::Client::new();
        let order = CreateOrdersRequest { orders };

        let create_order = client
            .post(format!("http://127.0.0.1:8080/table/{table_id}/order"))
            .header("Content-Type", "application/json")
            .json(&order)
            .send()
            .await
            .unwrap()
            .json::<ReturnTableResponse>()
            .await
            .unwrap();
        return create_order;
    }

    async fn delete_order_call(table_id: i64, order_id: i64) -> ReturnTableResponse {
        let client = reqwest::Client::new();

        let delete_order = client
            .delete(format!(
                "http://127.0.0.1:8080/table/{table_id}/order/{order_id}"
            ))
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap()
            .json::<ReturnTableResponse>()
            .await
            .unwrap();
        return delete_order;
    }

    //server one will delete the first order they see
    async fn server_one() {
        let table = create_table_call(["Gyoza".to_string()].to_vec()).await;
        let table_id = table.table.table_id;
        let create_orders = [
            create_order_call(table_id, ["Udon".to_string()].to_vec()),
            create_order_call(
                table_id,
                ["Hotdog".to_string(), "Borsht".to_string()].to_vec(),
            ),
        ];
        join_all(create_orders).await;
        let snap_shot = list_all_orders(None).await;
        //delete the first order you see
        if snap_shot.pagination.total > 0 && !snap_shot.orders.is_empty() {
            delete_order_call(table_id, snap_shot.orders[0].order_id).await;
        }
    }

    //server two will wait for a second and then "clean up tables" that have food done
    async fn server_two() {
        sleep(Duration::from_secs(10)).await;
        let list_order_query = Some(ListOrderQuery {
            cook_status: CookStatus::Done,
        });
        let snap_shot = list_all_orders(list_order_query).await;
        //delete the first order you see
        if snap_shot.pagination.total > 0 && !snap_shot.orders.is_empty() {
            delete_order_call(snap_shot.orders[0].table_id, snap_shot.orders[0].order_id).await;
        }
    }

    #[tokio::test]
    async fn run_async_test() {
        let mut serve_one = vec![];
        let mut serve_two = vec![];

        for i in 1..5 {
            if i % 2 == 0 {
                serve_one.push(tokio::spawn(async move { server_one() }))
            } else {
                serve_two.push(tokio::spawn(async move { server_two() }))
            }
        }

        let mut outputs = Vec::with_capacity(serve_one.len() + serve_two.len());

        for task in serve_one {
            let task = task.await.unwrap().await;

            outputs.push(task);
        }
        for task in serve_two {
            let task = task.await.unwrap().await;

            outputs.push(task);
        }
    }
}

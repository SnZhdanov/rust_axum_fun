#[cfg(test)]
pub mod integration_tests {
    use std::{thread, time::Duration};

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Response {
        tables: u64,
    }
    #[tokio::test]
    async fn run_test() {
        let mut arr = vec![];

        for i in 0..20 {
            arr.push(tokio::spawn(async move { call_post() }))
        }
        let mut outputs = Vec::with_capacity(arr.len());
        println!("Now we mix");
        for task in arr {
            let task = task.await.unwrap().await;

            outputs.push(task);
        }
        println!("{:?}", outputs);
    }

    async fn call_post() -> bool {
        let client = reqwest::Client::new();
        //try to call my api 10 times with 10 threads.
        println!("hello");
        match client
            .post("http://127.0.0.1:8080/table")
            .header("Content-Type", "application/json")
            .send()
            .await
        {
            Ok(resp) => {
                let resp = resp.json::<serde_json::Value>().await.unwrap();
                let resp: Response = serde_json::from_value(resp).unwrap();
                println!("{:?}", resp);
                true
            }
            Err(e) => {
                println!("{e}");
                false
            }
        }
    }
}

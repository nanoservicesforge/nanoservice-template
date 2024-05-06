//! You can run this test client against either the Actix or Axum server and you will get the same results.
use serde_json::{json, Value};
use reqwest::Client;
use kernel::{
    CalcWork,
    WorkType,
    Echo
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a reqwest client
    let client = Client::new();
    let base_url = "http://0.0.0.0:8001/api/v1";

    let calc_work_request = CalcWork {
        input_data1: 10,
        input_data2: 20,
        work_type: WorkType::Sum,
        result: None,
        error: None
    };

    // Send POST request to /calc endpoint
    let response = client
        .post(&format!("{}/calc", base_url))
        .json(&calc_work_request)
        .send()
        .await?;

    let response: CalcWork = response.json().await?;
    println!("CalcWork Response: {:?}", response);

    let echo_request = Echo {
        name: "World!".to_string(),
        error: None
    };

    // Send POST request to /echo endpoint
    let response = client
        .post(&format!("{}/echo", base_url))
        .json(&echo_request)
        .send()
        .await?;

    let response: Echo = response.json().await?;
    println!("Echo Response: {:?}", response);
    Ok(())
}

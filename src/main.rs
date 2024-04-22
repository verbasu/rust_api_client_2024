use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct RespJson {
    origin: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = Client::new();
    let response = client
        .post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;
    let result = response.json::<RespJson>().await?;
    println!("{:#?}", result);
    println!("origin: {}, url: {}", result.origin, result.url);
    Ok(())
}

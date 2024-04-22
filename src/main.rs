use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct DataJson {
    lang: String,
    body: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct HeadJson {
    // Content\-Type: String,
    // Content\-Length: String,
    Accept: String,
    Host: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct RespJson {
    headers: HeadJson,
    data: String,
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
    let data_json: DataJson = serde_json::from_str(&result.data).unwrap();
    // let head_json: HeadJson = serde_json::from_str(&result.headers).unwrap();
    println!("{:#?}", data_json);
    println!("{:#?}", result.headers);
    println!("origin: {}, url: {}", result.origin, result.url);
    Ok(())
}

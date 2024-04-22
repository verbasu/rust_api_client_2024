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
async fn make_post(url: &str) -> Result<RespJson, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    let client = Client::new();
    let response = client.post(url).json(&map).send().await?;
    let result = response.json::<RespJson>().await?;
    Ok(result)
}

fn main() {
    let url = "http://httpbin.org/post";
    let res = make_post(url).unwrap();
    let data_json: DataJson = serde_json::from_str(&res.data).unwrap();
    println!("{:#?}", data_json);
    println!("{:#?}", res.headers);
    println!("origin: {}, url: {}", res.origin, res.url);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_post_form_file() {
        let url = "http://httpbin.org/post";
        let res = make_post(url).unwrap();
    }
}

use reqwest::blocking::Client;
use std::fs::File;

fn upload_file(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("Cargo.toml")?;

    let client = Client::new();
    let res = client.post(url).body(file).send()?;

    Ok(())
}

fn main() {
    let url = "https://0x0.st";
    upload_file(url);
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_form_file() {
        let url = "https://0x0.st";
        let get_json = upload_file(url).await.unwrap();

        println!("Result: {:#?}", get_json);
    }
}
*/

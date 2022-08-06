use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook = "";

    let client = reqwest::Client::new();
    let res = client
        .post(webhook)
        .json(&json!({"content": "aaa"}))
        .send()
        .await?
        .text()
        .await?;

    println!("Body: {:?}", res);
    Ok(())
}

use clap::{App, Arg};
use serde_json::json;

fn main() {
    let app = App::new("send_my_address")
        .version("0.1.0")
        .about("send my address")
        .arg(
            Arg::new("webhook")
                .short('w')
                .long("webhook")
                .takes_value(true)
                .required(true),
        );

    match app.try_get_matches() {
        Ok(m) => {
            let webhook = m.value_of("webhook").unwrap();
            send(webhook);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[tokio::main]
async fn send(webhook: &str) -> Result<(), Box<dyn std::error::Error>> {
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

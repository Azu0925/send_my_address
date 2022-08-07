use clap::{App, Arg};
use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;
use serde::Serialize;
use serde_json::json;

fn main() {
    let network_interfaces = NetworkInterface::show().unwrap();
    let mut results: String = String::from("");
    for itf in network_interfaces.iter() {
        let address: &network_interface::Addr = &itf.addr.unwrap();
        let ip = match address {
            &network_interface::Addr::V4(v4_if_addr) => v4_if_addr.ip.to_string(),
            &network_interface::Addr::V6(v6_if_addr) => v6_if_addr.ip.to_string(),
        };
        let result = String::from("`name:") + &itf.name + ", address: " + &ip + "`\n";
        results.push_str(String::from(result).as_str());
    }

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
            println!("{}", results);
            send(webhook, results);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

#[tokio::main]
async fn send(webhook: &str, results: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .post(webhook)
        .json(&json!({ "content": &results }))
        .send()
        .await?
        .text()
        .await?;

    println!("Body: {:?}", res);
    Ok(())
}

#[derive(Serialize)]
struct Content {
    content: std::vec::Vec<String>,
}

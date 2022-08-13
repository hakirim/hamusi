use clap::Parser;
use tokio::main;
use serde_json;
use serde::{Serialize, Deserialize};

/// Conoha API Client
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// API user name
    #[clap(short, long, value_parser)]
    user_name: String,

    /// Tenant Id
    #[clap(short, long, value_parser)]
    tenant_id: String,

    /// Password
    #[clap(short, long, value_parser)]
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
struct Token {
    id: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
struct Access {
    token: Token
}

#[tokio::main]
async fn main() -> reqwest::Result<()>{
    //let conoha_api_page = "https://manage.conoha.jp/API/";
    let args = Args::parse();
    println!("{}, {}", args.user_name, args.tenant_id);
    let client = reqwest::Client::new();

    let send_result = client.post("https://identity.tyo1.conoha.io/v2.0/tokens")
        .header(reqwest::header::ACCEPT, "application/json")
        .body(format!("{{ \"auth\": {{ \"passwordCredentials\": {{ \"username\": \"{}\", \"password\": \"{}\" }}, \"tenantId\": \"{}\" }} }}", args.user_name, args.password, args.tenant_id)).send().await?.text().await?;
    println!("{}", send_result);
    let access: Access = serde_json::from_str(&send_result).unwrap();
    loop {
        let endpoint = gets();
    }
    Ok(())
}

fn gets() -> String {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _result = stdin.read_line(&mut buf);
    buf.clone()
}


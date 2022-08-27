use std::time;
use donoha::client::APITokenRequest;
use donoha::client::APIClient;
use donoha::types::{Servers};

fn main() {
    // DELETE ALL SERVER
    let user_name = std::env::var("DONOHA_KEY_USER_NAME").expect(format!("Please specify {}", "DONOHA_KEY_USER_NAME").as_str());
    let tenant_id = std::env::var("DONOHA_KEY_TENANT_ID").expect(format!("Please specify {}", "DONOHA_KEY_TENANT_ID").as_str());
    let password = std::env::var("DONOHA_KEY_PASSWORD").expect(format!("Please specify {}", "DONOHA_KEY_PASSWORD").as_str());
    let api_token_request = APITokenRequest::new(user_name, tenant_id.clone());
    let api_token = api_token_request.send(password).unwrap();
    let api_client = APIClient::new(api_token);
    let result = api_client.servers_text(tenant_id.clone()).unwrap();
    let servers: Servers = serde_json::from_str(&result).unwrap();
    for server in servers.servers {
        print!("shutdown...");
        if api_client.shutdown(&server) {
            println!("OK.");
        } else {
            println!("NG.");
        }
        print!("delete...");
        if api_client.delete(&server) {
            println!("OK.");
        } else {
            println!("NG.");
            println!("Retrying...");
            std::thread::sleep(time::Duration::from_secs(10));
            if api_client.delete(&server) {
                println!("OK.");
            } else {
                println!("NG.");
            }
        }
    }
    println!("DONE");
}
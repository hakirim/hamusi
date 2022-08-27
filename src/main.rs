use std::time;
use donoha::client::APITokenRequest;
use donoha::client::APIClient;
use donoha::types::Servers;
use std::collections::HashMap;

fn main() {
    menu_initial();
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

fn menu_initial() {
    let mut menu = HashMap::new();
    let menu_listing: fn() = menu_listing_server;
    let menu_e: fn() = menu_exit;
    menu.insert("1", menu_listing);
    menu.insert("0", menu_e);


    loop {
        println!("1. Listing servers");
        println!("0. Quit");
        let mut buf = String::new(); 
        let input = std::io::stdin();
        let _ = input.read_line(&mut buf);
        let str_in = buf.trim();
        let choice = menu.get(str_in);
        match choice {
            Some(f) => { f() }
            None => { () }
        }
    }

}

trait HamusiMenu {
}

fn menu_listing_server() {
    println!("This is menu_listing_server");
    println!("TODO implement");
    //let user_name = std::env::var("DONOHA_KEY_USER_NAME").expect(format!("Please specify {}", "DONOHA_KEY_USER_NAME").as_str());
    //let tenant_id = std::env::var("DONOHA_KEY_TENANT_ID").expect(format!("Please specify {}", "DONOHA_KEY_TENANT_ID").as_str());
    //let password = std::env::var("DONOHA_KEY_PASSWORD").expect(format!("Please specify {}", "DONOHA_KEY_PASSWORD").as_str());
    //let api_token_request = APITokenRequest::new(user_name, tenant_id.clone());
    //let api_token = api_token_request.send(password).unwrap();
    //let api_client = APIClient::new(api_token);
    //let result = api_client.servers_text(tenant_id.clone()).unwrap();
    //let servers: Servers = serde_json::from_str(&result).unwrap();
    //println!("number, server.id, server.name");
    //let mut number = 0;
    //for server in servers.servers {
    //    number += 1;
    //    println!("[{}] {}, {}", number, server.id, server.name);
    //}
    //println!("There are {} server/s.", number);
}

fn menu_exit() {
    println!("This is menu_exit");
    std::process::exit(0);
}


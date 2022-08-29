use donoha::client::APIClient;
use donoha::client::APIToken;
use donoha::client::APITokenRequest;
use donoha::types::Servers;
use std::collections::HashMap;
use std::time;

fn main() {
    menu_initial(None);
    // DELETE ALL SERVER
    let user_name = std::env::var("DONOHA_KEY_USER_NAME")
        .expect(format!("Please specify {}", "DONOHA_KEY_USER_NAME").as_str());
    let tenant_id = std::env::var("DONOHA_KEY_TENANT_ID")
        .expect(format!("Please specify {}", "DONOHA_KEY_TENANT_ID").as_str());
    let password = std::env::var("DONOHA_KEY_PASSWORD")
        .expect(format!("Please specify {}", "DONOHA_KEY_PASSWORD").as_str());
    let api_token_request = APITokenRequest::new(user_name, tenant_id.clone());
    let api_token = api_token_request.send(password).unwrap();
    let api_client = APIClient::new(&api_token);
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

fn menu_initial(token: Option<&APIToken>) {
    let mut menu = HashMap::new();
    let m_listing: fn(token: Option<&APIToken>) = menu_listing_server;
    let m_exit: fn(token: Option<&APIToken>) = menu_exit;
    menu.insert("1", m_listing);
    menu.insert("0", m_exit);

    loop {
        println!("1. Listing servers");
        println!("0. Quit");
        let mut buf = String::new();
        let input = std::io::stdin();
        let _ = input.read_line(&mut buf);
        let str_in = buf.trim();
        let choice = menu.get(str_in);
        match choice {
            Some(f) => f(token),
            None => (),
        }
    }
}

fn token_setup<'a>(check_token: Option<&'a APIToken>, mut result: &'a APIToken) {
    let response: &'a APIToken;
    match check_token {
        Some(token_ref) => result = token_ref,
        None => {
            println!("There are no token.");
            println!("");
            println!("### Get token");
            println!("");

            println!("Input Conoha API user name:");
            let mut buf = String::new();
            let input = std::io::stdin();
            let _ = input.read_line(&mut buf);
            let str_in = buf.trim();
            let in_user_name = str_in;
            let user_name = String::from(in_user_name);

            println!("Input Conoha API tenant id:");
            let mut buf = String::new();
            let input = std::io::stdin();
            let _ = input.read_line(&mut buf);
            let str_in = buf.trim();
            let in_user_name = str_in;
            let tenant_id = String::from(in_user_name);

            println!("Input Conoha API user name:");

            let new_password = "mNw9#2ae5TlKneEX";
            cli_clipboard::set_contents(new_password.to_owned()).unwrap();
            println!(
                "A random string copied to clipboard. Set this string to Conoha API password."
            );
            let req = APITokenRequest::new(user_name.clone(), tenant_id.clone());
            let response = req.send(new_password.to_string()).unwrap();
            result = &(response);
        }
    }
}

fn menu_listing_server(token: Option<&APIToken>) {
    let use_token_ref;
    token_setup(token, use_token_ref);
    println!("Input Conoha API tenant id:");
    let mut buf = String::new();
    let input = std::io::stdin();
    let _ = input.read_line(&mut buf);
    let str_in = buf.trim();
    let in_tenant_id = str_in;
    let tenant_id = Some(String::from(in_tenant_id));
    let api_client = APIClient::new(use_token_ref);
    let servers = api_client.servers(tenant_id.unwrap().clone()).unwrap();
    println!("number, server.id, server.name");
    let mut number = 0;
    for server in servers.servers {
        number += 1;
        println!("[{}] {}, {}", number, server.id, server.name);
    }
    println!("There are {} server/s.", number);
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
    ()
}

fn menu_exit(_token: Option<&APIToken>) {
    println!("This is menu_exit");
    std::process::exit(0);
}

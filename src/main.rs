use cli_clipboard::{ClipboardContext, ClipboardProvider};
use donoha::client::APIClient;
use donoha::client::APIToken;
use donoha::client::APITokenRequest;
//use donoha::types::Servers;
//use std::time;
//
//const DONOHA_KEY_USER_NAME: &str = "DONOHA_USER_NAME";
//const DONOHA_KEY_TENANT_ID: &str = "DONOHA_TENANT_ID";
//const DONOHA_KEY_PASSWORD: &str = "DONOHA_PASSWORD";
const DONOHA_KEY_API_TOKEN: &str = "DONOHA_API_TOKEN";
fn main() {
    // let opt_token: Option<APIToken> = std::env::var(DONOHA_KEY_API_TOKEN)
    //     .ok()
    //     .and_then(|value| Some(APIToken { value }));
    let opt_token = token_setup_from_env();
    println!("Input tenant id (NOT tenant name):");
    let tenant_id = gets();
    let token = match opt_token {
        Some(token) => token,
        None => loop {
            println!("Input user name:");
            let user_name = gets();
            let opt_token = generate_token(user_name, &tenant_id);
            match opt_token {
                Some(token) => {
                    println!("{}", token.value);
                    break token;
                }
                None => {}
            }
        },
    };
    let client = APIClient::new(&token);
    menu_initial(&client, &tenant_id);
    //    let api_client = APIClient::new(&api_token);
    //    let result = api_client.servers_text(tenant_id.clone()).unwrap();
    //    let servers: Servers = serde_json::from_str(&result).unwrap();
    //    for server in servers.servers {
    //        print!("shutdown...");
    //        if api_client.shutdown(&server) {
    //            println!("OK.");
    //        } else {
    //            println!("NG.");
    //        }
    //        print!("delete...");
    //        if api_client.delete(&server) {
    //            println!("OK.");
    //        } else {
    //            println!("NG.");
    //            println!("Retrying...");
    //            std::thread::sleep(time::Duration::from_secs(10));
    //            if api_client.delete(&server) {
    //                println!("OK.");
    //            } else {
    //                println!("NG.");
    //            }
    //        }
    //    }
    println!("DONE");
}

fn menu_initial(client: &APIClient, tenant_id: &String) {
    // メニューの選択肢
    loop {
        // メニュー表示
        println!("1. Listing servers");
        println!("0. Quit");

        // メニュー番号を入力させる
        let str_in = gets();
        let str_in = str_in.as_str();

        // 入力値をもとにメニューを処理する
        // 不正な入力値の場合、同じメニューを再表示する
        if str_in == "0" {
            std::process::exit(0);
        }
        if str_in == "1" {
            menu_listing_server(&client, tenant_id.clone())
        }

        ()
    }
}

fn gets() -> String {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn token_setup_from_env() -> Option<APIToken> {
    match std::env::var(DONOHA_KEY_API_TOKEN).ok() {
        Some(value) => Some(APIToken {
            value: value.trim().to_string(),
        }),
        None => None,
    }
}

fn generate_token(user_name: String, tenant_id: &String) -> Option<APIToken> {
    println!("Copy password to clipboad, and pless enter key. \n Hamusi will read password from clipboard.");
    let mut context = ClipboardContext::new().unwrap();
    gets();
    let password = context.get_contents().unwrap();
    APITokenRequest::new(user_name, (*tenant_id).clone())
        .send(password)
        .ok()
}

fn menu_listing_server(client: &APIClient, tenant_id: String) {
    // サーバー取得
    let servers = client.servers(tenant_id);
    match servers {
        Some(servers) => {
            println!("Number of servers: {}", servers.servers.len());
            for (i, server) in servers.servers.iter().enumerate() {
                if 0 == i {
                    println!("id, name");
                }
                println!("{},{}", server.id, server.name);
            }
        }
        None => {}
    }
}

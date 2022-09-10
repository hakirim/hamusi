use cli_clipboard::{ClipboardContext, ClipboardProvider};
use donoha::client::APIClient;
use donoha::client::APIToken;
use donoha::client::APITokenRequest;
use donoha::types::Server;

const DONOHA_KEY_TENANT_ID: &str = "DONOHA_TENANT_ID";
const DONOHA_KEY_API_TOKEN: &str = "DONOHA_API_TOKEN";
fn main() {
    let opt_token = token_setup_from_env();
    let tenant_id = match std::env::var(DONOHA_KEY_TENANT_ID) {
        Ok(tenant_id) => tenant_id,
        Err(_) => {
            println!("Input tenant id (NOT tenant name):");
            gets()
        }
    };
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
            menu_listing_server(&client, tenant_id)
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

fn menu_listing_server(client: &APIClient, tenant_id: &String) {
    // サーバー取得
    loop {
        let servers = client.servers((*tenant_id).clone()).unwrap();

        println!("");
        println!("Number of servers: {}", servers.servers.len());
        for (i, server) in servers.servers.iter().enumerate() {
            let no = i + 1;
            if 0 == i {
                println!("No, status, id, name");
            }
            println!("{}. {} {} {}", no, server.status, server.id, server.name);
        }
        println!("r. Reload list.");
        println!("0. Quit");

        println!("Input [No] of target server.");

        let str = gets();
        if str == "r" {
            continue;
        }
        if str == "0" {
            std::process::exit(0);
        }
        let no: usize = match str.parse() {
            Ok(n) => n,
            Err(_) => {
                continue;
            }
        };
        let server: &Server = match servers.servers.get(no - 1) {
            Some(s) => s,
            None => {
                continue;
            }
        };
        menu_control_server(client, &server);
    }
}

fn menu_control_server(client: &APIClient, server: &Server) {
    loop {
        // メニュー表示
        println!("b. Boot a server");
        println!("s. Stop a server");
        println!("d. Delete a server");
        println!("0. Back");

        // メニュー番号を入力させる
        let str = gets();
        let mut list_str = str.split_whitespace();
        let operation: &str = match list_str.nth(0) {
            Some(s) => s,
            None => {
                println!("invalid operation");
                continue;
            }
        };

        if operation == "b" {
            print!("Send boot request...");
            client.boot(server);
            println!("done.");
            return;
        } else if operation == "s" {
            print!("Send shutdown request...");
            client.shutdown(server);
            println!("done.");
            return;
        } else if operation == "d" {
            print!("Send delete request...");
            client.delete(server);
            println!("done.");
            return;
        } else if operation == "0" {
            println!("Back");
            return;
        } else {
            continue;
        }
    }
}

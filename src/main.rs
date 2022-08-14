use clap::Parser;

/// Conoha API Client
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// API user name
    #[clap(short, long, value_parser)]
    user_name: Option<String>,

    /// Password
    #[clap(short, long, value_parser)]
    password: Option<String>,
}

struct APIToken {
    value: String
}

struct TenantId {
    value: String
}

fn main() {

    // Try to get token from env
    let key_token = "HAMUSI_TOKEN";
    // Get tenant-id from env
    let key_tenant_id = "HAMUSI_TENANT_ID";
    let tenant_id = std::env::var(key_tenant_id).expect("Please specify HAMUSI_TENANT_ID");

    let token = match std::env::var(key_token) {
        Ok(val) => val,
        Err(_err) => {
            // Try to get token with args (user-name, password, tenant-id)
            let args = Args::parse();
            let user_name = args.user_name.expect("Please specify user-name.");
            let password = args.password.expect("Please specify password.");
            let client = reqwest::blocking::Client::new();
            let url_for_get_token = String::from("https://identity.tyo1.conoha.io/v2.0/tokens");

            // Parameters for get token
            let params = format!("{{ \"auth\": {{ \"passwordCredentials\": {{ \"username\": \"{}\", \"password\": \"{}\" }}, \"tenantId\": \"{}\" }} }}", user_name, password, tenant_id);

            let send_result = client.post(url_for_get_token)
                .header(reqwest::header::ACCEPT, "application/json")
                .body(params)
                .send().unwrap().text().unwrap();
            println!("{}", send_result);
            let splited: Vec<&str> = send_result.split('"').collect();
            splited[15].to_string()
        }

    };
    let client_and_token = ClientAndToken {
        client: reqwest::blocking::Client::new(),
        token: APIToken { value : token },
        tenant_id: TenantId { value: tenant_id }
    };

    match client_and_token.servers() {
        Some(result) => {
            println!("{}", result)
        }
        None => {
            println!("None")
        }
    }

    println!("END");
}

fn gets() -> String {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    let _result = stdin.read_line(&mut buf);
    buf.clone()
}

struct ClientAndToken {
    client: reqwest::blocking::Client,
    token: APIToken,
    tenant_id: TenantId
}

impl ClientAndToken {
    fn servers(&self) -> Option<String> {
        self.get(format!("https://compute.tyo1.conoha.io/v2/{}/servers", self.tenant_id.value))
    }

    fn get(&self, url: String) -> Option<String> {
        let send_result = self.client.get(url)
            .header(reqwest::header::ACCEPT, "application/json")
            .header("X-Auth-Token", &(self.token.value))
            .send().unwrap();
        if send_result.status().is_success() {
            Some(send_result.text().unwrap())
        } else {
            None
        }
    }
}

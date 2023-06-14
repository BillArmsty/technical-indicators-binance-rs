//Connect HTTP client that can send requests
use reqwest::Client;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

//Create a new HTTP client
pub fn get_client() -> Client {
    let client = Client::builder().user_agent(APP_USER_AGENT).build().unwrap();

    client
}

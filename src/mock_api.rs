use mockito::{Mock, Server, ServerGuard};

pub struct MockProvider {
    pub url: String,
    _server: ServerGuard,
    _mock: Mock,
}

async fn setup_mock(status: usize, body: &str, endpoint: &str) -> (ServerGuard, Mock, String) {
    let mut server = Server::new_async().await;

    let mock = server
        .mock("GET", endpoint)
        .with_status(status)
        .with_header("content-type", "application/json")
        .with_body(body)
        .create_async() // หรือ .create()
        .await;

    let url = server.url();

    (server, mock, url)
}

pub async fn get_url_a_provider() -> MockProvider {
    let mock_res = r#"{
        "symbol": "BTC",
        "name": "Bitcoin",
        "price": 1000000,
        "date": "2023-01-01"
    }"#;

    let endpoint = "/api/a_provider";
    let (server, mock, base_url) = setup_mock(200, mock_res, endpoint).await;

    let full_url = format!("{}{}", base_url, endpoint);

    MockProvider {
        url: full_url,
        _server: server,
        _mock: mock,
    }
}

pub async fn get_url_b_provider() -> MockProvider {
    let mock_res = r#"{
        "symbol": "ETH",
        "description": "Ethereum",
        "price": 50000
    }"#;

    let endpoint = "/api/b_provider";
    let (server, mock, base_url) = setup_mock(200, mock_res, endpoint).await;

    let full_url = format!("{}{}", base_url, endpoint);
    println!("{}", full_url);
    MockProvider {
        url: full_url,
        _server: server,
        _mock: mock,
    }
}

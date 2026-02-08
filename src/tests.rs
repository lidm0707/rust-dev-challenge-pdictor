use super::*;

// Simple Mock Monitor implementation for testing
#[derive(Debug, Default)]
struct MockMonitor;

impl MockMonitor {
    fn new() -> Self {
        Self::default()
    }
}

#[async_trait::async_trait]
impl Monitor for MockMonitor {
    async fn on_start(&self, symbol: &str) {
        println!("MockMonitor::on_start should not be called {}", symbol);
    }

    async fn on_finish(&self, symbol: &str, success: bool) {
        println!(
            "MockMonitor::on_finish should not be called {} is success {}",
            symbol, success
        );
    }
}

#[test]
fn test_error_display() {
    let error = FetchError::InvalidFormat("test".to_string());
    assert_eq!(format!("{}", error), "Invalid response format: test");
}

#[test]
fn test_price_data_deserialize() {
    let json = r#"{
        "symbol": "BTC",
        "price": 50000.0,
        "name": "Bitcoin",
        "description": "Digital Gold"
    }"#;
    let data: PriceData = serde_json::from_str(json).unwrap();
    assert_eq!(data.symbol, "BTC");
    assert_eq!(data.price, 50000.0);
    assert_eq!(data.name, Some("Bitcoin".to_string()));
    assert_eq!(data.description, Some("Digital Gold".to_string()));
}

#[test]
fn test_price_data_without_optional_fields() {
    let json = r#"{
        "symbol": "ETH",
        "price": 3000.0
    }"#;
    let data: PriceData = serde_json::from_str(json).unwrap();
    assert_eq!(data.symbol, "ETH");
    assert_eq!(data.price, 3000.0);
    assert_eq!(data.name, None);
    assert_eq!(data.description, None);
}

#[test]
fn test_config_creation() {
    let monitor = MockMonitor::new();
    let config = Config::new("http://example.com", &monitor);
    assert_eq!(config.base_url, "http://example.com");
}

#[test]
fn test_config_build_provider() {
    let monitor = MockMonitor::new();
    let config = Config::new("http://example.com", &monitor);
    let provider = config.build();
    assert_eq!(provider.base_url, "http://example.com");
}

#[cfg(feature = "a_provider")]
#[tokio::test]
async fn test_a_provider_fetch() {
    let server = get_url_a_provider().await;
    let monitor = MockMonitor::new();
    let url = server.url;
    let config = Config::new(&url, &monitor);
    let provider = config.build();
    let result = provider.fetch_price().await;

    println!("{:?}", result);
    let data = result.unwrap();
    assert_eq!(data.symbol, "BTC");
    assert_eq!(data.price, 1000000.0);
}

#[cfg(feature = "b_provider")]
#[tokio::test]
async fn test_b_provider_fetch() {
    let server = get_url_b_provider().await;
    let monitor = MockMonitor::new();
    let config = Config::new(&server.url, &monitor);
    let provider = config.build();
    let result = provider.fetch_price().await;

    println!("{:?}", result);
    let data = result.unwrap();
    assert_eq!(data.symbol, "ETH");
    assert_eq!(data.price, 50000.0);
}

use super::*;

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

#[cfg(feature = "a_provider")]
#[tokio::test]
async fn test_a_provider_fetch() {
    let server = get_url_a_provider().await;
    let provider = Provider::new_a(&server.url, Client::new());
    let result = provider.fetch_price(None).await;
    eprintln!("{:?}", &server.url);
    println!("{:?}", result);
    let data = result.unwrap();
    assert_eq!(data.symbol, "BTC");
    assert_eq!(data.price, 1000000.0);
}

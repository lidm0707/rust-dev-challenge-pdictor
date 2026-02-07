use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PriceData {
    pub symbol: String,
    pub price: f64,
    pub name: Option<String>,
    pub description: Option<String>,
}

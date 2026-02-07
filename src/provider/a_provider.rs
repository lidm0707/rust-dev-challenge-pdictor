#![cfg(feature = "a_provider")]
use super::{FetchResult, PriceData, Provider, ProviderTrait};

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub date: String,
}

#[async_trait::async_trait]
impl<'p> ProviderTrait for Provider<'p> {
    async fn fetch_price(&self) -> FetchResult<PriceData> {
        tokio::spawn(async move {
            monitor.clone().on_start().await;
        });
        let response = self.client.get(url).send().await?;
        self.monitor.log(response.status().as_str()).await;
        let data: Response = response.json().await?;
        self.monitor.on_finish().await;
        Ok(PriceData {
            symbol: data.symbol,
            price: data.price,
            name: Some(data.name),
            description: Some(format!("Date: {}", data.date)),
        })
    }
}

#![cfg(feature = "b_provider")]
use super::{FetchResult, Monitor, PriceData, Provider, ProviderTrait};

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub symbol: String,
    pub description: String,
    pub price: f64,
}

#[async_trait::async_trait]
impl<M: Monitor + Sync + Send + 'static> ProviderTrait for Provider<M> {
    async fn fetch_price(&self) -> FetchResult<PriceData> {
        let url = self.base_url;
        let monitor = self.monitor.clone();
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
            name: None,
            description: Some(data.description),
        })
    }
}

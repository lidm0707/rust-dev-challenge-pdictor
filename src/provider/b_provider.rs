#![cfg(feature = "b_provider")]
use super::{FetchResult, Monitor, PriceData, Provider, ProviderTrait};

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub symbol: String,
    pub description: String,
    pub price: f64,
}

#[async_trait::async_trait]
impl<'p, M: Monitor + Sync + Send + 'static> ProviderTrait for Provider<'p, M> {
    async fn fetch_price(&self) -> FetchResult<PriceData> {
        let url = self.base_url;
        let provider_name = self.provider_name;

        self.monitor.on_start(provider_name).await;
        let response = self.client.get(url).send().await?;
        let is_ok = response.status().is_success();
        let data: Response = response.json().await?;
        self.monitor.on_finish(provider_name, is_ok).await;

        Ok(PriceData {
            symbol: data.symbol,
            price: data.price,
            name: None,
            description: Some(data.description),
        })
    }
}

#![cfg(feature = "a_provider")]
use super::{FetchResult, Monitor, PriceData, Provider, ProviderTrait};
pub use uuid::Uuid;
#[derive(Debug, serde::Deserialize)]
pub struct Response {
    pub symbol: String,
    pub name: String,
    pub price: f64,
    pub date: String,
}

#[async_trait::async_trait]
impl<'p, M: Monitor + Sync + Send + 'static> ProviderTrait for Provider<'p, M> {
    async fn fetch_price(&self) -> FetchResult<PriceData> {
        let url = self.base_url;
        let provider_name = self.provider_name;
        let uuid = Uuid::new_v4();
        self.monitor.on_start(provider_name, uuid.as_bytes()).await;

        let response = self.client.get(url).send().await?;
        self.monitor
            .log(provider_name, uuid.as_bytes(), response.status().as_str())
            .await;
        let data: Response = response.json().await?;
        self.monitor.on_finish(provider_name, uuid.as_bytes()).await;
        Ok(PriceData {
            symbol: data.symbol,
            price: data.price,
            name: Some(data.name),
            description: Some(format!("Date: {}", data.date)),
        })
    }
}

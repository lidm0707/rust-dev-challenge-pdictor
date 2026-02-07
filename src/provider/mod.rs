use super::Monitor;
use super::{FetchResult, PriceData};
#[cfg(feature = "a_provider")]
pub mod a_provider;
#[cfg(feature = "b_provider")]
pub mod b_provider;
pub mod provider;
pub use provider::Provider;

#[async_trait::async_trait]
pub trait ProviderTrait {
    async fn fetch_price(&self) -> FetchResult<PriceData>;
}

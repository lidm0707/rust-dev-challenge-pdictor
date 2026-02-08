#[async_trait::async_trait]
pub trait Monitor {
    async fn on_start(&self, symbol: &str);
    async fn on_finish(&self, symbol: &str, success: bool);
}

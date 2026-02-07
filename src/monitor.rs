#[async_trait::async_trait]
pub trait Monitor {
    async fn on_start(&self);
    async fn on_finish(&self);
    async fn log(&self, message: &str);
}

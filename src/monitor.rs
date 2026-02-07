#[async_trait::async_trait]
pub trait Monitor {
    async fn on_start(&self, provuder: &str, uuid: &[u8]);
    async fn on_finish(&self, provuder: &str, uuid: &[u8]);
    async fn log(&self, provuder: &str, uuid: &[u8], message: &str);
}

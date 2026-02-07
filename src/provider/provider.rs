use std::sync::Arc;

use super::Monitor;
#[derive(Debug)]
pub struct Provider<M: Monitor + Sync + Send> {
    pub base_url: &'static str,
    pub client: reqwest::Client,
    pub monitor: Arc<M>,
}
impl<M: Monitor + Sync + Send> Provider<M> {
    pub fn new(base_url: &'static str, monitor: M) -> Self {
        Self {
            base_url: base_url,
            client: reqwest::Client::new(),
            monitor: Arc::new(monitor),
        }
    }
}

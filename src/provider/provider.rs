use std::sync::Arc;

use super::Monitor;
#[derive(Debug)]
pub struct Provider<'p, M: Monitor + Sync + Send> {
    pub base_url: &'p str,
    pub client: reqwest::Client,
    pub monitor: Arc<M>,
}
impl<'p, M: Monitor + Sync + Send> Provider<'p, M> {
    pub fn new(base_url: &'p str, monitor: M) -> Self {
        Self {
            base_url: base_url,
            client: reqwest::Client::new(),
            monitor: Arc::new(monitor),
        }
    }
}

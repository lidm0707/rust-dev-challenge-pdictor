use super::Monitor;
use super::provider::Provider;

pub struct Config<M: Monitor + Sync + Send> {
    pub base_url: &'static str,
    pub monitor: M,
}

impl<M: Monitor + Sync + Send> Config<M> {
    pub fn new(base_url: &'static str, monitor: M) -> Self {
        Self { base_url, monitor }
    }
    pub fn build(self) -> Provider<M> {
        Provider::new(self.base_url, self.monitor)
    }
}

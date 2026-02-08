use super::Monitor;
use super::provider::Provider;

pub struct Config<'c, M: Monitor + Sync + Send> {
    pub base_url: &'c str,
    pub monitor: &'c M,
}

impl<'c, M: Monitor + Sync + Send> Config<'c, M> {
    pub fn new(base_url: &'c str, monitor: &'c M) -> Self {
        Self { base_url, monitor }
    }
    pub fn build(self) -> Provider<'c, M> {
        Provider::new(self.base_url, self.monitor)
    }
}

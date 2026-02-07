use super::Monitor;
use super::provider::Provider;

pub struct Config<'c, M: Monitor + Sync + Send> {
    pub base_url: &'c str,
    pub monitor: M,
}

impl<'c, M: Monitor + Sync + Send> Config<'c, M> {
    pub fn new(base_url: &'c str, monitor: M) -> Self {
        Self { base_url, monitor }
    }
    pub fn build(self) -> Provider<'c, M> {
        Provider::new(self.base_url, self.monitor)
    }
}

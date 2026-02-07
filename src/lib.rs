#[cfg(not(any(feature = "a_provider", feature = "b_provider")))]
compile_error!(
    "At least one provider feature must be enabled. Use --features a_provider or --features b_provider"
);

#[cfg(all(feature = "a_provider", feature = "b_provider"))]
compile_error!(
    "Cannot enable both a_provider and b_provider features simultaneously. They are mutually exclusive."
);

pub mod mock_api;

#[cfg(test)]
mod tests;

pub mod config;
pub mod errors;

pub mod models;
pub mod monitor;
pub mod provider;

pub use config::*;
pub use errors::*;

pub use mock_api::*;
pub use models::*;
pub use monitor::*;
pub use provider::*;

pub type FetchResult<T> = Result<T, FetchError>;

/// Custom error enum for the library
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Invalid response format: {0}")]
    InvalidFormat(String),

    #[error("Timeout occurred")]
    Timeout,

    #[error("Provider not available")]
    ProviderUnavailable,
}

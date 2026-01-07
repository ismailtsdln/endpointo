use thiserror::Error;

/// Result type for Endpointo operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for Endpointo
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("XML serialization error: {0}")]
    XmlError(String),

    #[error("URL parse error: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Invalid input: {0}")]
    ValidationError(String),

    #[error("Parser error: {0}")]
    ParserError(String),

    #[error("Rate limit exceeded")]
    RateLimitError,

    #[error("Timeout error")]
    TimeoutError,

    #[error("TLS/SSL error: {0}")]
    TlsError(String),

    #[error("Encoding error: {0}")]
    EncodingError(String),

    #[error("Plugin error: {0}")]
    PluginError(String),

    #[error("robots.txt disallows crawling: {0}")]
    RobotsTxtError(String),

    #[error("Unknown error: {0}")]
    Other(String),
}

impl Error {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            Error::HttpError(_) | Error::TimeoutError | Error::RateLimitError
        )
    }

    /// Check if error is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(
            self,
            Error::HttpError(_) | Error::TimeoutError | Error::TlsError(_)
        )
    }
}

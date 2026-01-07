use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of API endpoint
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum EndpointType {
    Rest,
    GraphQL,
    WebSocket,
    Unknown,
}

/// Represents a discovered endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    /// The URL or path of the endpoint
    pub url: String,

    /// HTTP method (GET, POST, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Type of endpoint (REST, GraphQL, etc.)
    pub endpoint_type: EndpointType,

    /// Source file where the endpoint was found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Line number in source file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,

    /// Query parameters found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Vec<String>>,

    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl Endpoint {
    /// Create a new endpoint
    pub fn new(url: String, endpoint_type: EndpointType) -> Self {
        Self {
            url,
            method: None,
            endpoint_type,
            source: None,
            line: None,
            params: None,
            metadata: None,
        }
    }

    /// Set the HTTP method
    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Set the source file
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set the line number
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Add query parameters
    pub fn with_params(mut self, params: Vec<String>) -> Self {
        self.params = Some(params);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }
}

/// Scan result containing all discovered endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Target URL that was scanned
    pub target: String,

    /// Timestamp when scan started
    pub timestamp: String,

    /// Total endpoints found
    pub total_endpoints: usize,

    /// List of discovered endpoints
    pub endpoints: Vec<Endpoint>,

    /// Statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<ScanStats>,
}

/// Statistics about the scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStats {
    /// Total files processed
    pub files_processed: usize,

    /// Total requests made
    pub requests_made: usize,

    /// Duration in seconds
    pub duration_seconds: f64,

    /// Endpoints by type
    pub endpoints_by_type: HashMap<EndpointType, usize>,
}

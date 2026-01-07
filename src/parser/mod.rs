pub mod filters;
pub mod js_parser;
pub mod patterns;
pub mod sourcemap;

use crate::error::Result;
use crate::types::{Endpoint, EndpointType};
use js_parser::JsParser;
use patterns::PatternMatcher;
use tracing::{debug, info};

/// Main parser for extracting endpoints from web assets
pub struct Parser {
    js_parser: JsParser,
    pattern_matcher: PatternMatcher,
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            js_parser: JsParser::new(),
            pattern_matcher: PatternMatcher::new(),
        }
    }

    /// Parse JavaScript content and extract endpoints
    pub fn parse_js(&self, content: &str, source: Option<&str>) -> Result<Vec<Endpoint>> {
        info!("Parsing JavaScript ({}bytes)", content.len());

        let mut endpoints = Vec::new();

        // Extract URLs using regex patterns
        let urls = self.pattern_matcher.find_urls(content);

        for url in urls {
            let endpoint_type = self.determine_endpoint_type(&url);
            let mut endpoint = Endpoint::new(url.clone(), endpoint_type);

            if let Some(src) = source {
                endpoint = endpoint.with_source(src);
            }

            // Extract query parameters
            if let Some(params) = self.extract_params(&url) {
                endpoint = endpoint.with_params(params);
            }

            endpoints.push(endpoint);
        }

        // Extract API endpoints specifically
        endpoints.extend(self.pattern_matcher.find_api_endpoints(content, source));

        debug!("Found {} endpoints", endpoints.len());
        Ok(endpoints)
    }

    /// Determine the type of endpoint
    fn determine_endpoint_type(&self, url: &str) -> EndpointType {
        if url.contains("graphql") || url.contains("/gql") {
            EndpointType::GraphQL
        } else if url.starts_with("ws://") || url.starts_with("wss://") {
            EndpointType::WebSocket
        } else if url.contains("/api/") || url.contains("/v1/") || url.contains("/v2/") {
            EndpointType::Rest
        } else {
            EndpointType::Unknown
        }
    }

    /// Extract query parameters from URL
    fn extract_params(&self, url: &str) -> Option<Vec<String>> {
        if let Some(query_start) = url.find('?') {
            let query = &url[query_start + 1..];
            let params: Vec<String> = query
                .split('&')
                .filter_map(|p| p.split('=').next())
                .map(|s| s.to_string())
                .collect();

            if !params.is_empty() {
                return Some(params);
            }
        }
        None
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

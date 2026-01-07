pub mod filters;
pub mod js_parser;
pub mod patterns;
pub mod sourcemap;

use crate::error::Result;
use crate::types::Endpoint;
use js_parser::JsParser;
use patterns::PatternMatcher;
use tracing::{debug, info};

/// Main parser for extracting endpoints from web assets
pub struct Parser {
    _js_parser: JsParser,
    pattern_matcher: PatternMatcher,
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            _js_parser: JsParser::new(),
            pattern_matcher: PatternMatcher::new(),
        }
    }

    /// Parse JavaScript content and extract endpoints
    pub fn parse_js(&self, content: &str, source: Option<&str>) -> Result<Vec<Endpoint>> {
        info!("Parsing JavaScript ({} bytes)", content.len());

        let mut endpoints = Vec::new();

        // 1. Regex-based extraction using PatternMatcher
        let urls = self.pattern_matcher.find_urls(content);

        for url in urls {
            // Use PatternMatcher's type detection
            let endpoint_type = self.pattern_matcher.detect_endpoint_type(&url, content);
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

        // 2. Extract API endpoints specifically (with method detection)
        endpoints.extend(self.pattern_matcher.find_api_endpoints(content, source));

        // 3. Deduplicate endpoints
        let mut final_endpoints: Vec<Endpoint> = Vec::new();
        for ep in endpoints {
            if !final_endpoints
                .iter()
                .any(|e| e.url == ep.url && e.method == ep.method)
            {
                final_endpoints.push(ep);
            }
        }

        debug!("Found {} unique endpoints", final_endpoints.len());
        Ok(final_endpoints)
    }

    /// Extract query parameters from URL
    fn extract_params(&self, url: &str) -> Option<Vec<String>> {
        if let Some(query_start) = url.find('?') {
            let query = &url[query_start + 1..];
            let params: Vec<String> = query
                .split('&')
                .filter(|p| !p.is_empty())
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

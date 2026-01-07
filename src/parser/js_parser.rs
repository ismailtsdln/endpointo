use crate::parser::patterns::PatternMatcher;
use crate::types::{Endpoint, EndpointType};
use std::collections::HashMap;

/// JavaScript parser stub (to be replaced with tree-sitter or similar)
pub struct JsParser {
    pattern_matcher: PatternMatcher,
}

impl JsParser {
    /// Create a new JavaScript parser
    pub fn new() -> Self {
        Self {
            pattern_matcher: PatternMatcher::new(),
        }
    }

    /// Parse JavaScript content and extract endpoints
    pub fn parse(&self, content: &str) -> Vec<Endpoint> {
        let mut endpoints = Vec::new();

        // Regex-based extraction (Fallback until AST parser is ready)
        let urls = self.pattern_matcher.find_urls(content);
        for url in urls {
            let ep_type = self.pattern_matcher.detect_endpoint_type(&url, content);
            endpoints.push(Endpoint::new(url, ep_type));
        }

        // Detect GraphQL
        if content.contains("gql`") || content.contains("query {") {
            // Find possible GraphQL endpoints
            for line in content.lines() {
                if line.contains("/graphql") || line.contains("/v1/query") {
                    endpoints.push(Endpoint::new(
                        line.trim().to_string(),
                        EndpointType::GraphQL,
                    ));
                }
            }
        }

        endpoints
    }

    /// Check if content is minified
    pub fn is_minified(&self, content: &str) -> bool {
        if content.len() < 100 {
            return false;
        }

        let lines = content.lines().count();
        let avg_line_length = content.len() / lines;

        avg_line_length > 200 || lines < 5
    }
}

impl Default for JsParser {
    fn default() -> Self {
        Self::new()
    }
}

use crate::types::{Endpoint, EndpointType};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    // URL patterns
    static ref URL_REGEX: Regex = Regex::new(
        r#"(?:https?://|//)[^\s"'<>{}|\\\^`\[\]]+|/(?:api|v\d+|graphql)[^\s"'<>]*"#
    ).unwrap();

    // API endpoint patterns
    static ref API_PATTERNS: Vec<Regex> = vec![
        // REST API patterns
        Regex::new(r#"['"`]/(api|rest|v\d+)/[^'"`\s]+"#).unwrap(),
        Regex::new(r#"['"`]/[^'"`\s]*/(users?|auth|login|logout|register)[^'"`\s]*"#).unwrap(),

        // GraphQL patterns
        Regex::new(r#"['"`][^'"`\s]*graphql[^'"`\s]*"#).unwrap(),
        Regex::new(r#"mutation\s+\w+|query\s+\w+"#).unwrap(),

        // WebSocket patterns
        Regex::new(r#"wss?://[^\s"'<>{}|\\\^`\[\]]+"#).unwrap(),

        // Common API endpoints
        Regex::new(r#"['"`]/(data|fetch|submit|update|delete|create|get)[^'"`\s]*"#).unwrap(),
    ];

    // HTTP methods
    static ref HTTP_METHODS: Regex = Regex::new(
        r#"(?i)(GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS)\s*[,\(\s]"#
    ).unwrap();

    // Query parameter patterns
    static ref QUERY_PARAM: Regex = Regex::new(r#"\?([^&\s'"]+)"#).unwrap();
}

/// Pattern matcher for extracting endpoints from code
pub struct PatternMatcher;

impl PatternMatcher {
    pub fn new() -> Self {
        Self
    }

    /// Find all URLs in the content
    pub fn find_urls(&self, content: &str) -> Vec<String> {
        let mut urls = Vec::new();

        for cap in URL_REGEX.captures_iter(content) {
            if let Some(url) = cap.get(0) {
                let url_str = url
                    .as_str()
                    .trim_matches(|c| c == '"' || c == '\'' || c == '`');
                if !url_str.is_empty() && self.is_valid_url(url_str) {
                    urls.push(url_str.to_string());
                }
            }
        }

        // Deduplicate
        urls.sort();
        urls.dedup();
        urls
    }

    /// Find API-specific endpoints
    pub fn find_api_endpoints(&self, content: &str, source: Option<&str>) -> Vec<Endpoint> {
        let mut endpoints = Vec::new();

        for pattern in API_PATTERNS.iter() {
            for cap in pattern.captures_iter(content) {
                if let Some(matched) = cap.get(0) {
                    let url = matched
                        .as_str()
                        .trim_matches(|c| c == '"' || c == '\'' || c == '`');

                    let endpoint_type = if url.contains("graphql") {
                        EndpointType::GraphQL
                    } else if url.starts_with("ws") {
                        EndpointType::WebSocket
                    } else {
                        EndpointType::Rest
                    };

                    let mut endpoint = Endpoint::new(url.to_string(), endpoint_type);

                    // Try to find HTTP method
                    if let Some(method) = self.find_http_method_near(content, matched.start()) {
                        endpoint = endpoint.with_method(method);
                    }

                    if let Some(src) = source {
                        endpoint = endpoint.with_source(src);
                    }

                    endpoints.push(endpoint);
                }
            }
        }

        endpoints
    }

    /// Find HTTP method near a position in the content
    fn find_http_method_near(&self, content: &str, pos: usize) -> Option<String> {
        // Look backward and forward 100 characters
        let start = pos.saturating_sub(100);
        let end = std::cmp::min(pos + 100, content.len());
        let snippet = &content[start..end];

        HTTP_METHODS
            .captures(snippet)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_uppercase())
    }

    /// Validate if a string is a valid URL
    fn is_valid_url(&self, url: &str) -> bool {
        // Filter out common false positives
        if url.len() < 4 {
            return false;
        }

        // Exclude common file extensions that are not endpoints
        let extensions = [".jpg", ".png", ".gif", ".css", ".woff", ".ttf", ".ico"];
        for ext in &extensions {
            if url.ends_with(ext) {
                return false;
            }
        }

        true
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_urls() {
        let matcher = PatternMatcher::new();
        let content = r#"
            fetch("https://api.example.com/users");
            axios.get("/api/v1/posts");
        "#;

        let urls = matcher.find_urls(content);
        assert!(urls.contains(&"https://api.example.com/users".to_string()));
        assert!(urls.contains(&"/api/v1/posts".to_string()));
    }

    #[test]
    fn test_graphql_detection() {
        let matcher = PatternMatcher::new();
        let content = r#"query getUser { user { id name } }"#;

        let endpoints = matcher.find_api_endpoints(content, None);
        assert!(!endpoints.is_empty());
    }
}

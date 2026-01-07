/// Filters for endpoint results
pub struct EndpointFilter;

impl EndpointFilter {
    pub fn new() -> Self {
        Self
    }

    /// Apply filter to endpoint URL
    pub fn matches(&self, url: &str, pattern: &str) -> bool {
        // Simple substring matching
        // TODO: Support regex patterns
        url.contains(pattern)
    }

    /// Deduplicate endpoints
    pub fn deduplicate(&self, endpoints: &mut Vec<crate::types::Endpoint>) {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        endpoints.retain(|e| seen.insert(e.url.clone()));
    }
}

impl Default for EndpointFilter {
    fn default() -> Self {
        Self::new()
    }
}

use crate::error::Result;
use crate::types::{Endpoint, EndpointType};
use sourcemap::SourceMap;
use tracing::{debug, warn};

/// Sourcemap extractor and resolver
pub struct SourceMapExtractor;

impl SourceMapExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Extract sourcemap URL from JavaScript content
    pub fn extract_sourcemap_url(&self, content: &str) -> Option<String> {
        // Look for sourceMappingURL comment
        for line in content.lines().rev() {
            if let Some(url_start) = line.find("sourceMappingURL=") {
                let url = &line[url_start + 17..].trim();
                return Some(url.to_string());
            }
        }
        None
    }

    /// Parse sourcemap content
    pub fn parse_sourcemap(&self, content: &str) -> Result<Vec<Endpoint>> {
        let mut endpoints = Vec::new();
        debug!("Parsing sourcemap content...");

        match SourceMap::from_reader(content.as_bytes()) {
            Ok(sm) => {
                for (i, source) in sm.sources().enumerate() {
                    if let Some(source_content) = sm.get_source_contents(i as u32) {
                        debug!("Analyzing source map file: {}", source);
                        // Mark the sources
                        let ep = Endpoint::new(source.to_string(), EndpointType::Unknown)
                            .with_source("sourcemap".to_string());
                        endpoints.push(ep);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to parse sourcemap: {}", e);
            }
        }

        Ok(endpoints)
    }
}

impl Default for SourceMapExtractor {
    fn default() -> Self {
        Self::new()
    }
}

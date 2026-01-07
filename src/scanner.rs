use crate::config::ScanConfig;
use crate::crawler::Crawler;
use crate::error::Result;
use crate::parser::Parser;
use crate::types::Endpoint;
use std::path::Path;
use tokio::fs;
use tracing::{error, info};

/// Main scanner orchestrator
pub struct Scanner {
    crawler: Crawler,
    parser: Parser,
    config: ScanConfig,
}

impl Scanner {
    /// Create a new scanner
    pub fn new(config: ScanConfig) -> Self {
        let crawler = Crawler::new(config.clone()).expect("Failed to create crawler");
        let parser = Parser::new();

        Self {
            crawler,
            parser,
            config,
        }
    }

    /// Scan a URL and extract endpoints
    pub async fn scan_url(&self, url: &str) -> Result<Vec<Endpoint>> {
        info!("Starting scan of {}", url);

        let mut all_endpoints = Vec::new();

        // Crawl the URL to find JavaScript assets
        let assets = self.crawler.crawl(url).await?;
        info!("Found {} JavaScript assets", assets.len());

        // Parse each asset
        for asset_url in assets {
            match self.crawler.fetch_js(&asset_url).await {
                Ok(js_content) => match self.parser.parse_js(&js_content, Some(&asset_url)) {
                    Ok(endpoints) => {
                        info!("Extracted {} endpoints from {}", endpoints.len(), asset_url);
                        all_endpoints.extend(endpoints);
                    }
                    Err(e) => {
                        error!("Failed to parse {}: {}", asset_url, e);
                    }
                },
                Err(e) => {
                    error!("Failed to fetch {}: {}", asset_url, e);
                }
            }
        }

        // Also parse the main page
        if let Ok(html) = self.crawler.fetch_js(url).await {
            if let Ok(endpoints) = self.parser.parse_js(&html, Some(url)) {
                all_endpoints.extend(endpoints);
            }
        }

        // Apply filter if specified
        if let Some(filter) = &self.config.filter_pattern {
            all_endpoints.retain(|e| e.url.contains(filter));
        }

        info!("Total endpoints found: {}", all_endpoints.len());
        Ok(all_endpoints)
    }

    /// Parse a local file and extract endpoints
    pub async fn parse_file(&self, path: &Path) -> Result<Vec<Endpoint>> {
        info!("Parsing file: {}", path.display());

        let content = fs::read_to_string(path).await?;
        let source = path.to_string_lossy().to_string();

        self.parser.parse_js(&content, Some(&source))
    }
}

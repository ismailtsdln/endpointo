pub mod client;
pub mod rate_limiter;
pub mod robots;

use crate::config::ScanConfig;
use crate::error::Result;
use crate::types::Endpoint;
use client::HttpClient;
use dashmap::DashSet;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};
use url::Url;

/// Async web crawler for discovering JavaScript assets
pub struct Crawler {
    client: Arc<HttpClient>,
    config: ScanConfig,
    visited: Arc<DashSet<String>>,
    semaphore: Arc<Semaphore>,
}

impl Crawler {
    /// Create a new crawler
    pub fn new(config: ScanConfig) -> Result<Self> {
        let client = Arc::new(HttpClient::new(&config)?);
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        let visited = Arc::new(DashSet::new());

        Ok(Self {
            client,
            config,
            visited,
            semaphore,
        })
    }

    /// Crawl a URL and discover assets
    pub async fn crawl(&self, url: &str) -> Result<Vec<String>> {
        let parsed_url = Url::parse(url)?;

        info!("Starting crawl of {}", url);

        // Check robots.txt if enabled
        if self.config.respect_robots_txt {
            if !self.client.check_robots_txt(&parsed_url).await? {
                warn!("robots.txt disallows crawling {}", url);
                return Ok(Vec::new());
            }
        }

        let mut assets = Vec::new();

        // Fetch the main page
        if let Ok(html) = self.fetch_html(url).await {
            // Extract script tags
            assets.extend(self.extract_scripts(&html, &parsed_url));
        }

        Ok(assets)
    }

    /// Fetch HTML content from a URL
    async fn fetch_html(&self, url: &str) -> Result<String> {
        if self.visited.contains(url) {
            debug!("Already visited {}", url);
            return Ok(String::new());
        }

        self.visited.insert(url.to_string());

        // Acquire semaphore permit for concurrency control
        let _permit = self.semaphore.acquire().await.unwrap();

        debug!("Fetching {}", url);
        self.client.get(url).await
    }

    /// Extract script sources from HTML
    fn extract_scripts(&self, html: &str, base_url: &Url) -> Vec<String> {
        let mut scripts = Vec::new();

        // Simple regex-based extraction (can be improved with HTML parser)
        let script_regex = regex::Regex::new(r#"<script[^>]+src=["']([^"']+)["']"#).unwrap();

        for cap in script_regex.captures_iter(html) {
            if let Some(src) = cap.get(1) {
                let script_url = src.as_str();

                // Resolve relative URLs
                if let Ok(absolute_url) = base_url.join(script_url) {
                    scripts.push(absolute_url.to_string());
                }
            }
        }

        scripts
    }

    /// Fetch JavaScript content
    pub async fn fetch_js(&self, url: &str) -> Result<String> {
        if self.visited.contains(url) {
            return Ok(String::new());
        }

        self.visited.insert(url.to_string());
        let _permit = self.semaphore.acquire().await.unwrap();

        debug!("Fetching JavaScript {}", url);
        self.client.get(url).await
    }
}

use crate::cli::InteractiveUi;
use crate::config::ScanConfig;
use crate::crawler::Crawler;
use crate::error::Result;
use crate::parser::Parser;
use crate::plugins::PluginManager;
use crate::types::Endpoint;
use std::path::Path;
use tokio::fs;
use tracing::{error, info};

/// Main scanner orchestrator
pub struct Scanner {
    crawler: Crawler,
    parser: Parser,
    config: ScanConfig,
    plugin_manager: PluginManager,
    ui: Option<InteractiveUi>,
}

impl Scanner {
    /// Create a new scanner
    pub fn new(config: ScanConfig) -> Self {
        let crawler = Crawler::new(config.clone()).expect("Failed to create crawler");
        let parser = Parser::new();
        let mut plugin_manager = PluginManager::new();

        if let Some(plugin_path) = &config.plugin_path {
            let _ = plugin_manager.load_plugin(plugin_path);
        }

        Self {
            crawler,
            parser,
            config,
            plugin_manager,
            ui: None,
        }
    }

    /// Set interactive UI
    pub fn with_ui(mut self, ui: InteractiveUi) -> Self {
        self.ui = Some(ui);
        self
    }

    /// Scan a URL and extract endpoints
    pub async fn scan_url(&self, url: &str) -> Result<Vec<Endpoint>> {
        info!("Starting scan of {}", url);

        if let Some(ui) = &self.ui {
            ui.set_main_message(&format!("Scanning {}", url));
        }

        let mut all_endpoints = Vec::new();

        // Crawl the URL to find JavaScript assets
        let assets = self.crawler.crawl(url).await?;
        info!("Found {} JavaScript assets", assets.len());

        if let Some(ui) = &self.ui {
            ui.inc_main();
            ui.set_main_message(&format!("Found {} JS files", assets.len()));
        }

        // Parse each asset
        for asset_url in assets {
            if let Some(ui) = &self.ui {
                ui.set_main_message(&format!("Parsing {}", asset_url));
            }

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

        // Apply plugins
        let mut processed_endpoints = Vec::new();
        for ep in all_endpoints {
            if self.plugin_manager.filter_endpoint(&ep) {
                let transformed = self.plugin_manager.transform_endpoint(ep);
                processed_endpoints.push(transformed);
            }
        }

        // Apply config-based filter if specified
        if let Some(filter) = &self.config.filter_pattern {
            processed_endpoints.retain(|e| e.url.contains(filter));
        }

        if let Some(ui) = &self.ui {
            ui.finish();
        }

        info!("Total endpoints found: {}", processed_endpoints.len());
        Ok(processed_endpoints)
    }

    /// Parse a local file and extract endpoints
    pub async fn parse_file(&self, path: &Path) -> Result<Vec<Endpoint>> {
        info!("Parsing file: {}", path.display());

        let content = fs::read_to_string(path).await?;
        let source = path.to_string_lossy().to_string();

        let endpoints = self.parser.parse_js(&content, Some(&source))?;

        // Apply plugins
        let mut processed_endpoints = Vec::new();
        for ep in endpoints {
            if self.plugin_manager.filter_endpoint(&ep) {
                let transformed = self.plugin_manager.transform_endpoint(ep);
                processed_endpoints.push(transformed);
            }
        }

        Ok(processed_endpoints)
    }
}

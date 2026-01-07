//! # Endpointo
//!
//! High-performance API endpoint discovery tool for security professionals.
//!
//! This library provides the core functionality for extracting API endpoints,
//! URLs, and paths from modern web assets including JavaScript files, bundles,
//! and sourcemaps.
//!
//! ## Features
//!
//! - Async HTTP/HTTPS crawling with rate limiting
//! - JavaScript and asset parsing (minified, bundled, sourcemap-enabled)
//! - Multiple output formats (JSON, YAML, XML, HTML)
//! - Plugin architecture for extensibility
//! - robots.txt compliance
//! - TLS/SSL error handling
//!
//! ```no_run
//! use endpointo::scanner::Scanner;
//! use endpointo::config::ScanConfig;
//! use endpointo::types::Endpoint;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = ScanConfig::new("https://example.com".to_string());
//!     let scanner = Scanner::new(config)?;
//!     
//!     let results: Vec<Endpoint> = scanner.scan_url("https://example.com").await?;
//!     println!("Found {} endpoints", results.len());
//!     
//!     Ok(())
//! }
//! ```

pub mod cli;
pub mod config;
pub mod crawler;
pub mod error;
pub mod output;
pub mod parser;
pub mod scanner;
pub mod types;

pub mod plugins;

#[cfg(feature = "python-plugins")]
pub mod bridge;

// Re-export commonly used types
pub use error::{Error, Result};
pub use scanner::Scanner;
pub use types::{Endpoint, EndpointType, ScanResult};

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "endpointo",
    version,
    about = "High-performance API endpoint discovery tool",
    long_about = "Endpointo is a modern security tool for extracting API endpoints, URLs, and paths from web assets.\n\nDesigned for security professionals, bug bounty hunters, and penetration testers."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Scan a URL and extract endpoints
    #[command(name = "scan")]
    Scan {
        /// Target URL to scan
        #[arg(short, long, value_name = "URL")]
        url: String,

        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Output format
        #[arg(short, long, value_enum, default_value = "json")]
        format: Option<OutputFormat>,

        /// Rate limit (requests per second)
        #[arg(short, long, value_name = "NUM")]
        rate_limit: Option<u32>,

        /// Request timeout in seconds
        #[arg(short, long, value_name = "SECS", default_value = "30")]
        timeout: Option<u64>,

        /// Number of concurrent threads
        #[arg(short = 'j', long, value_name = "NUM", default_value = "10")]
        threads: Option<usize>,

        /// Filter pattern for endpoints
        #[arg(long, value_name = "PATTERN")]
        filter: Option<String>,

        /// Python plugin to load
        #[arg(short, long, value_name = "PATH")]
        plugin: Option<PathBuf>,
    },

    /// Parse local JavaScript files
    #[command(name = "parse")]
    Parse {
        /// Input files (glob patterns supported)
        #[arg(short, long, value_name = "FILES", required = true)]
        files: Vec<PathBuf>,

        /// Output file path
        #[arg(short, long, value_name = "FILE")]
        output: Option<PathBuf>,

        /// Output format
        #[arg(short = 'F', long, value_enum, default_value = "json")]
        format: Option<OutputFormat>,

        /// Filter pattern for endpoints
        #[arg(long, value_name = "PATTERN")]
        filter: Option<String>,

        /// Python plugin to load
        #[arg(short, long, value_name = "PATH")]
        plugin: Option<PathBuf>,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    /// JSON format
    Json,
    /// YAML format
    Yaml,
    /// XML format
    Xml,
    /// HTML report
    Html,
}

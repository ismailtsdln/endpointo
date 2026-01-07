use anyhow::Result;
use clap::Parser as _;
use endpointo::cli::{Cli, Commands, InteractiveUi};
use endpointo::config::ScanConfig;
use endpointo::output::{write_results, OutputFormat};
use endpointo::scanner::Scanner;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Parse CLI arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            url,
            output,
            format,
            rate_limit,
            timeout,
            threads,
            filter,
            plugin,
        } => {
            let mut config = ScanConfig::new(url.clone())
                .with_rate_limit(rate_limit)
                .with_timeout(timeout)
                .with_max_concurrent(threads);

            if let Some(f) = filter {
                config = config.with_filter(f);
            }

            if let Some(p) = plugin {
                config = config.with_plugin(PathBuf::from(p));
            }

            let mut scanner = Scanner::new(config);

            // Use interactive UI if verbose logging is not enabled and stdout is a terminal
            if std::env::var("RUST_LOG").is_err() {
                scanner = scanner.with_ui(InteractiveUi::new(5));
            }

            let results = scanner.scan_url(&url).await?;
            let output_format = format.unwrap_or(OutputFormat::Json);
            write_results(&results, output.as_deref(), output_format)?;

            println!("✅ Scan complete! Found {} endpoints", results.len());
            if let Some(output_path) = output {
                println!("📄 Results saved to: {}", output_path.display());
            }
        }

        Commands::Parse {
            files,
            output,
            format,
            filter,
            plugin,
        } => {
            let mut config = ScanConfig::default();
            if let Some(f) = filter {
                config = config.with_filter(f);
            }
            if let Some(p) = plugin {
                config = config.with_plugin(PathBuf::from(p));
            }

            let scanner = Scanner::new(config);

            let mut all_results = Vec::new();
            for file in files {
                let results = scanner.parse_file(&file).await?;
                all_results.extend(results);
            }

            // Write output
            let output_format = format.unwrap_or(OutputFormat::Json);
            write_results(&all_results, output.as_deref(), output_format)?;

            println!("✅ Parse complete! Found {} endpoints", all_results.len());
            if let Some(output_path) = output {
                println!("📄 Results saved to: {}", output_path.display());
            }
        }
    }

    Ok(())
}

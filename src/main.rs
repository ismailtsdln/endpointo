use anyhow::Result;
use clap::Parser;
use endpointo::cli::{Cli, Commands};
use endpointo::config::ScanConfig;
use endpointo::output::OutputFormat;
use endpointo::scanner::Scanner;
use tracing_subscriber::{fmt, EnvFilter};

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
            let config = ScanConfig {
                rate_limit: rate_limit.unwrap_or(10),
                timeout_seconds: timeout.unwrap_or(30),
                max_concurrent: threads.unwrap_or(10),
                follow_redirects: true,
                respect_robots_txt: true,
                user_agent: Some("Endpointo/0.1.0".to_string()),
                filter_pattern: filter,
            };

            let scanner = Scanner::new(config);
            let results = scanner.scan_url(&url).await?;

            // Write output
            let output_format = format.unwrap_or(OutputFormat::Json);
            endpointo::output::write_results(&results, output.as_deref(), output_format)?;

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
            let config = ScanConfig::default();
            let scanner = Scanner::new(config);

            let mut all_results = Vec::new();
            for file in files {
                let results = scanner.parse_file(&file).await?;
                all_results.extend(results);
            }

            // Apply filter if specified
            if let Some(filter_pattern) = filter {
                all_results.retain(|endpoint| {
                    endpoint.url.contains(&filter_pattern)
                        || endpoint.method.as_deref().unwrap_or("").contains(&filter_pattern)
                });
            }

            // Write output
            let output_format = format.unwrap_or(OutputFormat::Json);
            endpointo::output::write_results(&all_results, output.as_deref(), output_format)?;

            println!("✅ Parse complete! Found {} endpoints", all_results.len());
            if let Some(output_path) = output {
                println!("📄 Results saved to: {}", output_path.display());
            }
        }
    }

    Ok(())
}

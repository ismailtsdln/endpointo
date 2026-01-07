use anyhow::Result;
use clap::Parser as _;
use colored::*;
use endpointo::cli::{Cli, Commands, InteractiveUi};
use endpointo::config::ScanConfig;
use endpointo::output::{write_results, OutputFormat};
use endpointo::scanner::Scanner;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

fn print_banner() {
    let banner = r#"
    _______  __    _  ______   _______  _______  ___   __    _  _______  _______ 
|       ||  |  | ||      | |       ||       ||   | |  |  | ||       ||       |
|    ___||   |_| ||  _    ||    _  ||   _   ||   | |   |_| ||    ___||   _   |
|   |___ |       || | |   ||   |_| ||  | |  ||   | |       ||   | __ |  | |  |
|    ___||  _    || |_|   ||    ___||  |_|  ||   | |  _    ||   ||  ||  |_|  |
|   |___ | | |   ||       ||   |    |       ||   | | | |   ||   |_| ||       |
|_______||_|  |__||______| |___|    |_______||___| |_|  |__||_______||_______|
    "#;
    println!("{}", banner.bright_cyan().bold());
    println!(
        "  {} v{}\n",
        "API Endpoint Discovery Tool".italic().dimmed(),
        "0.1.0".bright_green()
    );
}

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

    print_banner();

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
            println!(
                "{} {}...",
                "🚀 Starting scan of".bright_white(),
                url.bold().bright_blue()
            );

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

            let mut scanner = Scanner::new(config)?;

            // Use interactive UI if verbose logging is not enabled and stdout is a terminal
            if std::env::var("RUST_LOG").is_err() {
                scanner = scanner.with_ui(InteractiveUi::new(5));
            }

            let results: Vec<endpointo::types::Endpoint> = scanner.scan_url(&url).await?;
            let output_format = format.unwrap_or(OutputFormat::Json);
            write_results(&results, output.as_deref(), output_format)?;

            println!(
                "\n{} Found {} endpoints",
                "✅ Scan complete!".bright_green().bold(),
                results.len().to_string().bold()
            );
            if let Some(output_path) = output {
                println!(
                    "{} {}",
                    "📄 Results saved to:".dimmed(),
                    output_path.display().to_string().bright_white().underline()
                );
            }
        }

        Commands::Parse {
            files,
            output,
            format,
            filter,
            plugin,
        } => {
            println!(
                "{} {} files...",
                "📂 Parsing".bright_white(),
                files.len().to_string().bold().bright_blue()
            );

            let mut config = ScanConfig::default();
            if let Some(f) = filter {
                config = config.with_filter(f);
            }
            if let Some(p) = plugin {
                config = config.with_plugin(PathBuf::from(p));
            }

            let scanner = Scanner::new(config)?;

            let mut all_results: Vec<endpointo::types::Endpoint> = Vec::new();
            for file in files {
                match scanner.parse_file(&file).await {
                    Ok(results) => all_results.extend(results),
                    Err(e) => eprintln!("{} {}: {}", "❌ Error parsing".red(), file.display(), e),
                }
            }

            // Write output
            let output_format = format.unwrap_or(OutputFormat::Json);
            write_results(&all_results, output.as_deref(), output_format)?;

            println!(
                "\n{} Parsed {} endpoints",
                "✅ Parse complete!".bright_green().bold(),
                all_results.len().to_string().bold()
            );
            if let Some(output_path) = output {
                println!(
                    "{} {}",
                    "📄 Results saved to:".dimmed(),
                    output_path.display().to_string().bright_white().underline()
                );
            }
        }
    }

    Ok(())
}

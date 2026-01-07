# 🧠 Endpointo

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

> **High-performance API endpoint discovery tool for security professionals and bug bounty hunters**

Endpointo is a modern, blazing-fast CLI tool designed to extract API endpoints, URLs, and paths from web assets. Built with Rust for maximum performance and Python for extensibility, it handles minified, bundled, and sourcemap-enabled JavaScript files with ease.

## 🚀 Features

- **⚡ High-Speed Crawling**: Async HTTP engine with concurrent request processing
- **🔍 Smart Parsing**: Regex and pattern-based endpoint extraction from modern JS frameworks
- **🗺️ Sourcemap Support**: Extract and resolve endpoints from minified code
- **📊 Multiple Output Formats**: JSON, YAML, XML, and HTML reports
- **🔌 Plugin Architecture**: Extend functionality with Python plugins
- **🛡️ Security-Focused**: robots.txt compliance, TLS verification, and DoS protection
- **🎯 API Type Detection**: Automatically classify REST, GraphQL, and WebSocket endpoints
- **⚙️ Highly Configurable**: Rate limiting, timeouts, concurrency control, and filtering

## 📦 Installation

### Binary Release (Recommended)

Download the latest release for your platform:

```bash
# Linux
curl -LO https://github.com/ismailtsdln/endpointo/releases/latest/download/endpointo-linux-x64
chmod +x endpointo-linux-x64
sudo mv endpointo-linux-x64 /usr/local/bin/endpointo

# macOS
curl -LO https://github.com/ismailtsdln/endpointo/releases/latest/download/endpointo-macos-x64
chmod +x endpointo-macos-x64
sudo mv endpointo-macos-x64 /usr/local/bin/endpointo

# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/ismailtsdln/endpointo/releases/latest/download/endpointo-windows-x64.exe" -OutFile "endpointo.exe"
```

### From Source

```bash
# Prerequisites: Rust 1.70+
git clone https://github.com/ismailtsdln/endpointo.git
cd endpointo
cargo build --release
sudo cp target/release/endpointo /usr/local/bin/
```

### Cargo Install

```bash
cargo install endpointo
```

## 🎯 Quick Start

### Scan a URL

Extract API endpoints from a live website:

```bash
endpointo scan -u https://example.com -o results.json
```

### Parse Local Files

Analyze JavaScript files without making network requests:

```bash
endpointo parse -f ./assets/*.js --format html -o report.html
```

### Advanced Filtering

Find specific endpoint patterns:

```bash
endpointo scan -u https://api.example.com --filter "/api/" --format yaml
```

## 📚 Usage

### Scan Command

```bash
endpointo scan [OPTIONS] --url <URL>

OPTIONS:
  -u, --url <URL>              Target URL to scan
  -o, --output <FILE>          Output file path (prints to stdout if not specified)
  -f, --format <FORMAT>        Output format [default: json] [possible values: json, yaml, xml, html]
  -r, --rate-limit <NUM>       Requests per second [default: 10]
  -t, --timeout <SECS>         Request timeout in seconds [default: 30]
  -j, --threads <NUM>          Number of concurrent threads [default: 10]
      --filter <PATTERN>       Filter endpoints by pattern
  -p, --plugin <PATH>          Load Python plugin
  -h, --help                   Print help
```

### Parse Command

```bash
endpointo parse [OPTIONS] --files <FILES>...

OPTIONS:
  -f, --files <FILES>...       Input files (glob patterns supported)
  -o, --output <FILE>          Output file path
  -F, --format <FORMAT>        Output format [default: json] [possible values: json, yaml, xml, html]
      --filter <PATTERN>       Filter endpoints by pattern
  -p, --plugin <PATH>          Load Python plugin
  -h, --help                   Print help
```

## 🔧 Examples

### Basic Scan

```bash
# Scan a target and save results as JSON
endpointo scan -u https://target.com -o endpoints.json

# Scan with custom rate limiting and timeout
endpointo scan -u https://target.com -r 5 -t 60 -o results.json
```

### Output Formats

```bash
# JSON output (default)
endpointo scan -u https://target.com --format json

# YAML output
endpointo scan -u https://target.com --format yaml -o endpoints.yaml

# HTML report
endpointo scan -u https://target.com --format html -o report.html

# XML output
endpointo scan -u https://target.com --format xml -o endpoints.xml
```

### File Parsing

```bash
# Parse single file
endpointo parse -f app.min.js

# Parse multiple files with glob
endpointo parse -f "static/**/*.js" -o all_endpoints.json

# Parse with filter
endpointo parse -f bundle.js --filter "api" --format yaml
```

### Advanced Usage

```bash
# High-concurrency scan
endpointo scan -u https://target.com -j 50 -r 100

# Filter specific endpoints
endpointo scan -u https://api.example.com --filter "/v1/" -o api_v1.json

# Use custom plugin
endpointo scan -u https://target.com --plugin ./plugins/custom_filter.py
```

## 🔌 Plugin System

Endpointo supports Python plugins for custom filtering and analysis. See the [Plugin Development Guide](./plugins/README.md) for details.

### Example Plugin

```python
# plugins/custom_filter.py
def filter_endpoint(endpoint):
    """Filter endpoints based on custom logic"""
    return "/admin/" not in endpoint["url"]

def transform_endpoint(endpoint):
    """Transform endpoint data"""
    endpoint["custom_field"] = "value"
    return endpoint
```

## 🏗️ Architecture

```
endpointo/
├── src/
│   ├── cli/          # Command-line interface
│   ├── crawler/      # HTTP crawler with rate limiting
│   ├── parser/       # JavaScript and endpoint parser
│   ├── output/       # Output formatters (JSON/YAML/XML/HTML)
│   ├── scanner.rs    # Main scan orchestrator
│   ├── config.rs     # Configuration management
│   ├── types.rs      # Core data structures
│   └── error.rs      # Error handling
├── plugins/          # Python plugin system
├── tests/            # Integration tests
└── docs/             # Documentation
```

## 🛡️ Security

Endpointo is designed with security in mind:

- **TLS Verification**: Enforces certificate validation (no invalid certs accepted)
- **Rate Limiting**: Built-in DoS protection with configurable limits
- **robots.txt Compliance**: Respects website crawling policies
- **Input Validation**: Sanitizes all user inputs to prevent injection attacks
- **Error Handling**: Graceful degradation with detailed error messages

For security vulnerabilities, please see [SECURITY.md](./SECURITY.md).

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration_test

# Run all tests with output
cargo test -- --nocapture

# Lint and format
cargo clippy
cargo fmt
```

## 📊 Performance

Endpointo is optimized for speed:

- Async I/O with Tokio runtime
- Concurrent request processing
- Memory-efficient streaming parsers
- Zero-copy parsing where possible

**Benchmark**: Scanning a medium-sized SPA (~50 JS files) takes **< 5 seconds** on average hardware.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## 🙏 Credits

Created by [Ismail Tasdelen](https://github.com/ismailtsdln) for the security research community.

**Built with:**
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Tokio](https://tokio.rs/) - Async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [Clap](https://github.com/clap-rs/clap) - CLI framework

## 📮 Support

- 🐛 [Report a Bug](https://github.com/ismailtsdln/endpointo/issues/new?labels=bug)
- 💡 [Request a Feature](https://github.com/ismailtsdln/endpointo/issues/new?labels=enhancement)
- 💬 [Ask a Question](https://github.com/ismailtsdln/endpointo/discussions)

---

<p align="center">Made with ❤️ for the bug bounty community</p>

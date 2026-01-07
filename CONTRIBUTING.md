# Contributing to Endpointo

Thank you for your interest in contributing to Endpointo! This document provides guidelines and instructions for contributing.

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

## How to Contribute

### Reporting Bugs

Before creating a bug report:
1. Check existing issues to avoid duplicates
2. Collect information about the bug (OS, Rust version, error messages)
3. Create a minimal reproduction case

When filing a bug report, include:
- **Description**: Clear description of the issue
- **Steps to Reproduce**: Minimal steps to reproduce the behavior
- **Expected Behavior**: What you expected to happen
- **Actual Behavior**: What actually happened
- **Environment**: OS, Rust version, Endpointo version
- **Logs**: Relevant error messages or logs

### Suggesting Features

Feature requests are welcome! Include:
- **Use Case**: Why is this feature needed?
- **Proposed Solution**: How should it work?
- **Alternatives**: Other approaches considered
- **Additional Context**: Screenshots, examples, etc.

### Pull Requests

1. **Fork** the repository
2. **Create a branch** from `main`:
   ```bash
   git checkout -b feature/my-feature
   ```
3. **Make your changes** following our coding standards
4. **Add tests** for new functionality
5. **Run tests** and linting:
   ```bash
   cargo test
   cargo clippy --all-targets --all-features
   cargo fmt -- --check
   ```
6. **Commit** with clear messages:
   ```bash
   git commit -m "feat: add endpoint deduplication"
   ```
7. **Push** to your fork:
   ```bash
   git push origin feature/my-feature
   ```
8. **Open a Pull Request** with:
   - Clear title and description
   - Link to related issues
   - Screenshots/examples if applicable

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- (Optional) Python 3.8+ for plugin development

### Setup

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/endpointo.git
cd endpointo

# Add upstream remote
git remote add upstream https://github.com/ismailtsdln/endpointo.git

# Build the project
cargo build

# Run tests
cargo test
```

### Running Locally

```bash
# Run in development mode
cargo run -- scan -u https://example.com

# Build release version
cargo build --release
./target/release/endpointo --help
```

## Coding Standards

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write idiomatic Rust code
- Add documentation comments for public APIs

### Code Organization

- Keep functions small and focused
- Use descriptive variable and function names
- Avoid deep nesting (max 3-4 levels)
- Prefer composition over inheritance
- Handle errors explicitly (avoid unwrap in library code)

### Documentation

- Add rustdoc comments for all public items:
  ```rust
  /// Parses JavaScript content and extracts endpoints
  ///
  /// # Arguments
  ///
  /// * `content` - The JavaScript source code
  /// * `source` - Optional source file path
  ///
  /// # Returns
  ///
  /// A vector of discovered endpoints
  pub fn parse_js(&self, content: &str, source: Option<&str>) -> Result<Vec<Endpoint>>
  ```
- Include examples in documentation
- Update README.md for user-facing changes

### Testing

- Write unit tests for new functions
- Add integration tests for new features
- Aim for >80% code coverage
- Use descriptive test names:
  ```rust
  #[test]
  fn test_parse_graphql_endpoints() {
      // ...
  }
  ```

### Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `style:` Code formatting (no logic changes)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

Examples:
```
feat: add GraphQL endpoint detection
fix: handle UTF-8 encoding errors in parser
docs: update README with installation instructions
```

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_graphql_endpoints

# Run integration tests only
cargo test --test integration_test
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_creation() {
        let endpoint = Endpoint::new(
            "/api/users".to_string(),
            EndpointType::Rest
        );
        
        assert_eq!(endpoint.url, "/api/users");
        assert_eq!(endpoint.endpoint_type, EndpointType::Rest);
    }
}
```

## Building for Release

```bash
# Build optimized binary
cargo build --release

# Cross-compile for Linux
cargo build --release --target x86_64-unknown-linux-gnu

# Cross-compile for Windows
cargo build --release --target x86_64-pc-windows-msvc
```

## Plugin Development

See [plugins/README.md](./plugins/README.md) for plugin development guidelines.

## Project Structure

```
endpointo/
├── src/
│   ├── cli/          # CLI command definitions
│   ├── crawler/      # HTTP crawling logic
│   ├── parser/       # Endpoint parsing
│   ├── output/       # Output formatters
│   ├── scanner.rs    # Main scanner
│   ├── config.rs     # Configuration
│   ├── types.rs      # Data structures
│   └── error.rs      # Error types
├── plugins/          # Python plugins
├── tests/            # Integration tests
├── docs/             # Documentation
└── benches/          # Benchmarks
```

## Getting Help

- 💬 [GitHub Discussions](https://github.com/ismailtsdln/endpointo/discussions) - Ask questions
- 🐛 [GitHub Issues](https://github.com/ismailtsdln/endpointo/issues) - Report bugs
- 📧 Email: ismailtasdelen@gmail.com

## Recognition

Contributors are recognized in:
- Release notes
- GitHub contributors page
- README acknowledgments (for significant contributions)

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Endpointo! 🚀

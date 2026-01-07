# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of Endpointo seriously. If you discover a security vulnerability, please follow these steps:

### 1. **DO NOT** Disclose Publicly

Please do not open a public GitHub issue for security vulnerabilities. This helps protect users who may be using vulnerable versions.

### 2. Contact Us Privately

Send details of the vulnerability to: **[ismailtasdelen@gmail.com]**

Include in your report:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if available)

### 3. Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**: Depends on severity (critical issues prioritized)

### 4. Disclosure Process

1. We acknowledge receipt of your vulnerability report
2. We confirm the vulnerability and determine its severity
3. We develop and test a fix
4. We release a patched version
5. We publicly disclose the vulnerability (with credit to reporter, if desired)

## Security Best Practices for Users

When using Endpointo, follow these security guidelines:

### 1. Rate Limiting

Always use appropriate rate limiting to avoid overwhelming target servers:

```bash
# Use moderate rate limits (default is 10 req/s)
endpointo scan -u https://target.com -r 10
```

### 2. Respect robots.txt

Endpointo respects `robots.txt` by default. Do not disable this unless you have explicit permission:

```bash
# robots.txt compliance is enabled by default
endpointo scan -u https://target.com
```

### 3. Use with Authorization

Only scan:
- Your own applications
- Applications where you have written permission
- Bug bounty programs within their scope

### 4. TLS Verification

Endpointo enforces TLS certificate verification. This cannot be disabled, ensuring secure connections.

### 5. Plugin Security

When using Python plugins:
- Only use plugins from trusted sources
- Review plugin code before execution
- Be aware plugins have access to scan data

### 6. Sensitive Data

Endpointo may extract endpoints containing sensitive information:
- Store output files securely
- Do not commit endpoints.json to public repositories
- Redact sensitive data before sharing

## Known Security Considerations

### Attack Surface

Endpointo's attack surface includes:

1. **HTTP Client**: Uses `reqwest` for HTTP operations
   - Mitigation: Keep dependencies updated, enforce TLS verification

2. **Regex Engine**: Uses `regex` crate for pattern matching
   - Mitigation: Patterns are pre-compiled and tested for ReDoS

3. **Python FFI** (optional): PyO3 bridge for plugins
   - Mitigation: Plugin feature is optional and isolated

4. **File I/O**: Reads JavaScript files from disk
   - Mitigation: Validates file paths and handles errors gracefully

5. **Output Serialization**: JSON/YAML/XML/HTML generation
   - Mitigation: Uses well-maintained serialization crates, escapes output

### DoS Protection

Endpointo includes built-in protections against accidental DoS:

- Rate limiting (default: 10 req/s)
- Concurrent request limits (default: 10)
- Request timeouts (default: 30s)
- Exponential backoff on errors

## Vulnerability Severity Guidelines

We use the following severity levels:

- **Critical**: Remote code execution, authentication bypass
- **High**: Significant data exposure, privilege escalation
- **Medium**: Moderate data exposure, DoS vulnerabilities
- **Low**: Minor information disclosure, configuration issues

## Security Updates

Security patches are released as soon as possible after verification. Users are notified via:

- GitHub Security Advisories
- Release notes
- Project README

## Responsible Disclosure

We believe in responsible disclosure and will:

- Work with reporters to understand and fix issues
- Provide credit to reporters (unless they prefer anonymity)
- Coordinate disclosure timing to protect users
- Issue CVEs for significant vulnerabilities

## Legal Notice

Security testing should only be performed on:

1. Systems you own
2. Systems where you have explicit written permission
3. Bug bounty programs within their defined scope

Unauthorized security testing may be illegal in your jurisdiction.

## Contact

For security-related questions or concerns:

- Email: ismailtasdelen@gmail.com
- GitHub: [@ismailtsdln](https://github.com/ismailtsdln)

---

Thank you for helping keep Endpointo and its users safe!

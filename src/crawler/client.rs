use crate::config::ScanConfig;
use crate::error::{Error, Result};
use governor::{Quota, RateLimiter as GovernorLimiter};
use nonzero_ext::nonzero;
use reqwest::{header, Client};
use robotstxt::DefaultMatcher;
use std::time::Duration;
use tracing::{debug, warn};
use url::Url;

/// HTTP client with rate limiting and retry logic
pub struct HttpClient {
    client: Client,
    rate_limiter: GovernorLimiter<
        governor::state::direct::NotKeyed,
        governor::state::InMemoryState,
        governor::clock::DefaultClock,
    >,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new(config: &ScanConfig) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        if let Some(ua) = &config.user_agent {
            headers.insert(
                header::USER_AGENT,
                header::HeaderValue::from_str(ua)
                    .map_err(|e| Error::ValidationError(format!("Invalid user agent: {}", e)))?,
            );
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .redirect(if config.follow_redirects {
                reqwest::redirect::Policy::limited(10)
            } else {
                reqwest::redirect::Policy::none()
            })
            .default_headers(headers)
            .danger_accept_invalid_certs(false) // Enforce TLS verification
            .build()?;

        // Configure rate limiter
        let quota = Quota::per_second(nonzero!(config.rate_limit));
        let rate_limiter = GovernorLimiter::direct(quota);

        Ok(Self {
            client,
            rate_limiter,
        })
    }

    /// Perform GET request with rate limiting
    pub async fn get(&self, url: &str) -> Result<String> {
        // Wait for rate limiter
        self.rate_limiter.until_ready().await;

        debug!("Making GET request to {}", url);

        let response = self.client.get(url).send().await.map_err(|e| {
            if e.is_timeout() {
                Error::TimeoutError
            } else if e.is_connect() {
                Error::TlsError(format!("Connection error: {}", e))
            } else {
                Error::HttpError(e)
            }
        })?;

        // Check status code
        if !response.status().is_success() {
            warn!("HTTP {} for {}", response.status(), url);
            return Err(Error::HttpError(reqwest::Error::from(
                response.error_for_status().unwrap_err(),
            )));
        }

        // Read response body with encoding detection
        let bytes = response.bytes().await?;
        let (content, _, had_errors) = encoding_rs::UTF_8.decode(&bytes);

        if had_errors {
            warn!("Encoding errors detected in response from {}", url);
        }

        Ok(content.into_owned())
    }

    /// Check robots.txt for URL
    pub async fn check_robots_txt(&self, url: &Url) -> Result<bool> {
        let robots_url = format!(
            "{}://{}/robots.txt",
            url.scheme(),
            url.host_str().unwrap_or("")
        );

        debug!("Checking robots.txt at {}", robots_url);

        // Fetch robots.txt
        let robots_content = match self.get(&robots_url).await {
            Ok(content) => content,
            Err(Error::HttpError(_)) => {
                // No robots.txt, allow crawling
                debug!("No robots.txt found, allowing crawl");
                return Ok(true);
            }
            Err(e) => return Err(e),
        };

        // Parse robots.txt
        let matcher = DefaultMatcher::default();
        let user_agent = "Endpointo";

        // Simple check - can be improved
        let allowed = !robots_content.contains(&format!("Disallow: {}", url.path()));

        if !allowed {
            warn!("robots.txt disallows {}", url);
        }

        Ok(allowed)
    }
}

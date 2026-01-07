/// Configuration for scanning operations
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Maximum requests per second
    pub rate_limit: u32,

    /// Timeout for HTTP requests in seconds
    pub timeout_seconds: u64,

    /// Maximum concurrent requests
    pub max_concurrent: usize,

    /// Follow HTTP redirects
    pub follow_redirects: bool,

    /// Respect robots.txt
    pub respect_robots_txt: bool,

    /// Custom User-Agent header
    pub user_agent: Option<String>,

    /// Filter pattern for endpoints
    pub filter_pattern: Option<String>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            rate_limit: 10,
            timeout_seconds: 30,
            max_concurrent: 10,
            follow_redirects: true,
            respect_robots_txt: true,
            user_agent: Some("Endpointo/0.1.0".to_string()),
            filter_pattern: None,
        }
    }
}

impl ScanConfig {
    /// Create a new configuration with custom settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Set rate limit
    pub fn rate_limit(mut self, rate: u32) -> Self {
        self.rate_limit = rate;
        self
    }

    /// Set timeout
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }

    /// Set maximum concurrent requests
    pub fn max_concurrent(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Enable/disable following redirects
    pub fn follow_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    /// Enable/disable robots.txt compliance
    pub fn respect_robots_txt(mut self, respect: bool) -> Self {
        self.respect_robots_txt = respect;
        self
    }

    /// Set custom user agent
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Set filter pattern
    pub fn filter(mut self, pattern: impl Into<String>) -> Self {
        self.filter_pattern = Some(pattern.into());
        self
    }
}

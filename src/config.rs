use std::path::PathBuf;

/// Configuration for scanning operations
#[derive(Debug, Clone)]
pub struct ScanConfig {
    /// Target URL or input source
    pub target_url: String,

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

    /// Path to a Python plugin
    pub plugin_path: Option<PathBuf>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            target_url: String::new(),
            rate_limit: 10,
            timeout_seconds: 30,
            max_concurrent: 10,
            follow_redirects: true,
            respect_robots_txt: true,
            user_agent: Some("Endpointo/0.1.0".to_string()),
            filter_pattern: None,
            plugin_path: None,
        }
    }
}

impl ScanConfig {
    /// Create a new configuration with custom settings
    pub fn new(target_url: String) -> Self {
        Self {
            target_url,
            ..Self::default()
        }
    }

    /// Set rate limit
    pub fn with_rate_limit(mut self, rate: Option<u32>) -> Self {
        if let Some(r) = rate {
            self.rate_limit = r;
        }
        self
    }

    /// Set timeout
    pub fn with_timeout(mut self, seconds: Option<u64>) -> Self {
        if let Some(s) = seconds {
            self.timeout_seconds = s;
        }
        self
    }

    /// Set maximum concurrent requests
    pub fn with_max_concurrent(mut self, max: Option<usize>) -> Self {
        if let Some(m) = max {
            self.max_concurrent = m;
        }
        self
    }

    /// Enable/disable following redirects
    pub fn with_redirects(mut self, follow: bool) -> Self {
        self.follow_redirects = follow;
        self
    }

    /// Enable/disable robots.txt compliance
    pub fn with_robots(mut self, respect: bool) -> Self {
        self.respect_robots_txt = respect;
        self
    }

    /// Set custom user agent
    pub fn with_user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Set filter pattern
    pub fn with_filter(mut self, pattern: String) -> Self {
        self.filter_pattern = Some(pattern);
        self
    }

    /// Set plugin path
    pub fn with_plugin(mut self, path: PathBuf) -> Self {
        self.plugin_path = Some(path);
        self
    }
}

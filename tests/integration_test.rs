use std::fs;
use tempfile::tempdir;

#[tokio::test]
async fn test_parse_simple_js() {
    // Create a temp directory with test file
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test.js");

    let js_content = r#"
        fetch("https://api.example.com/users");
        axios.get("/api/v1/posts");
        const endpoint = "/api/comments";
    "#;

    fs::write(&file_path, js_content).unwrap();

    let config = endpointo::config::ScanConfig::default();
    let scanner = endpointo::Scanner::new(config).unwrap();

    let results = scanner.parse_file(&file_path).await.unwrap();

    // Should find at least 2 endpoints
    assert!(
        results.len() >= 2,
        "Expected at least 2 endpoints, found {}",
        results.len()
    );

    // Check if we found the API endpoints
    let urls: Vec<String> = results.iter().map(|e| e.url.clone()).collect();
    assert!(urls.iter().any(|u| u.contains("api.example.com/users")));
    assert!(urls.iter().any(|u| u.contains("/api/v1/posts")));
}

#[tokio::test]
async fn test_graphql_detection() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("graphql.js");

    let js_content = r#"
        const query = gql`
            query GetUser {
                user {
                    id
                    name
                }
            }
        `;
        
        fetch("/graphql", {
            method: "POST",
            body: JSON.stringify({ query })
        });
    "#;

    fs::write(&file_path, js_content).unwrap();

    let config = endpointo::config::ScanConfig::default();
    let scanner = endpointo::Scanner::new(config).unwrap();

    let results = scanner.parse_file(&file_path).await.unwrap();

    // Should detect GraphQL endpoint
    assert!(results
        .iter()
        .any(|e| { matches!(e.endpoint_type, endpointo::types::EndpointType::GraphQL) }));
}

#[test]
fn test_endpoint_type_detection() {
    use endpointo::types::{Endpoint, EndpointType};

    let rest_endpoint = Endpoint::new("/api/v1/users".to_string(), EndpointType::Rest);
    assert_eq!(rest_endpoint.endpoint_type, EndpointType::Rest);

    let graphql_endpoint = Endpoint::new("/graphql".to_string(), EndpointType::GraphQL);
    assert_eq!(graphql_endpoint.endpoint_type, EndpointType::GraphQL);
}

#[test]
fn test_endpoint_builder() {
    use endpointo::types::Endpoint;
    use endpointo::types::EndpointType;

    let endpoint = Endpoint::new("/api/users".to_string(), EndpointType::Rest)
        .with_method("GET")
        .with_source("app.js")
        .with_line(42);

    assert_eq!(endpoint.url, "/api/users");
    assert_eq!(endpoint.method, Some("GET".to_string()));
    assert_eq!(endpoint.source, Some("app.js".to_string()));
    assert_eq!(endpoint.line, Some(42));
}

#[test]
fn test_config_builder() {
    use endpointo::config::ScanConfig;

    let config = ScanConfig::new("https://example.com".to_string())
        .with_rate_limit(Some(20))
        .with_timeout(Some(60))
        .with_max_concurrent(Some(5))
        .with_user_agent("CustomAgent/1.0");

    assert_eq!(config.rate_limit, 20);
    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.max_concurrent, 5);
    assert_eq!(config.user_agent, Some("CustomAgent/1.0".to_string()));
}

"""
Regex Filter Plugin for Endpointo

This plugin filters endpoints based on regex patterns.
Only endpoints matching the configured patterns will be included.
"""

import re

# Configuration: Add your regex patterns here
API_PATTERNS = [
    r'/api/v\d+/',      # Versioned API endpoints
    r'/graphql',        # GraphQL endpoints
    r'/rest/',          # REST endpoints
]

# Compile patterns once for performance
COMPILED_PATTERNS = [re.compile(pattern) for pattern in API_PATTERNS]


def filter_endpoint(endpoint):
    """
    Filter endpoints based on regex patterns.
    
    Args:
        endpoint (dict): Endpoint data with keys: url, method, endpoint_type, etc.
        
    Returns:
        bool: True if endpoint matches any pattern, False otherwise
    """
    url = endpoint.get('url', '')
    
    # Check if URL matches any pattern
    for pattern in COMPILED_PATTERNS:
        if pattern.search(url):
            return True
    
    return False


def transform_endpoint(endpoint):
    """
    Add metadata about which pattern matched.
    
    Args:
        endpoint (dict): Endpoint data
        
    Returns:
        dict: Endpoint with added metadata
    """
    url = endpoint.get('url', '')
    
    for i, pattern in enumerate(COMPILED_PATTERNS):
        if pattern.search(url):
            if 'metadata' not in endpoint:
                endpoint['metadata'] = {}
            endpoint['metadata']['matched_pattern'] = API_PATTERNS[i]
            break
    
    return endpoint


# Example usage
if __name__ == '__main__':
    test_endpoints = [
        {'url': '/api/v1/users', 'endpoint_type': 'rest'},
        {'url': '/static/main.js', 'endpoint_type': 'unknown'},
        {'url': '/graphql', 'endpoint_type': 'graphql'},
        {'url': '/home', 'endpoint_type': 'unknown'},
    ]
    
    print("Testing regex filter...")
    for ep in test_endpoints:
        included = filter_endpoint(ep)
        print(f"  {ep['url']}: {'✓ INCLUDED' if included else '✗ FILTERED'}")

"""
URL Deduplication Plugin for Endpointo

This plugin provides advanced URL deduplication by normalizing
URLs and removing duplicates based on path and parameter structure.
"""

from urllib.parse import urlparse, parse_qs, urlencode
import hashlib


def normalize_url(url):
    """
    Normalize a URL for deduplication.
    
    This function:
    - Removes parameter values (keeps keys)
    - Sorts query parameters
    - Normalizes paths
    
    Args:
        url (str): Original URL
        
    Returns:
        str: Normalized URL
    """
    try:
        parsed = urlparse(url)
        
        # Normalize path (remove trailing slash)
        path = parsed.path.rstrip('/')
        
        # Parse and sort query parameters (remove values)
        if parsed.query:
            params = parse_qs(parsed.query)
            # Keep only parameter keys, sorted
            param_keys = sorted(params.keys())
            normalized_query = '&'.join([f"{key}=" for key in param_keys])
        else:
            normalized_query = ''
        
        # Rebuild normalized URL
        scheme = parsed.scheme or ''
        netloc = parsed.netloc or ''
        
        if scheme and netloc:
            normalized = f"{scheme}://{netloc}{path}"
        else:
            normalized = path
        
        if normalized_query:
            normalized += f"?{normalized_query}"
        
        return normalized
        
    except Exception:
        # If parsing fails, return original URL
        return url


def url_fingerprint(url):
    """
    Generate a fingerprint hash for a URL.
    
    Args:
        url (str): URL to fingerprint
        
    Returns:
        str: MD5 hash of normalized URL
    """
    normalized = normalize_url(url)
    return hashlib.md5(normalized.encode()).hexdigest()


def transform_endpoint(endpoint):
    """
    Add normalized URL and fingerprint to endpoint.
    
    Args:
        endpoint (dict): Endpoint data
        
    Returns:
        dict: Endpoint with normalization metadata
    """
    url = endpoint.get('url', '')
    
    if 'metadata' not in endpoint:
        endpoint['metadata'] = {}
    
    endpoint['metadata']['normalized_url'] = normalize_url(url)
    endpoint['metadata']['url_fingerprint'] = url_fingerprint(url)
    
    return endpoint


def deduplicate_endpoints(endpoints):
    """
    Deduplicate a list of endpoints.
    
    Args:
        endpoints (list): List of endpoint dicts
        
    Returns:
        list: Deduplicated endpoints
    """
    seen_fingerprints = set()
    unique_endpoints = []
    
    for endpoint in endpoints:
        fingerprint = url_fingerprint(endpoint.get('url', ''))
        
        if fingerprint not in seen_fingerprints:
            seen_fingerprints.add(fingerprint)
            unique_endpoints.append(endpoint)
    
    return unique_endpoints


# Example usage
if __name__ == '__main__':
    test_urls = [
        '/api/users?id=1&name=john',
        '/api/users?name=jane&id=2',  # Same structure
        '/api/users?id=3',  # Different params
        '/api/posts/',
        '/api/posts',  # Trailing slash variation
    ]
    
    print("URL Normalization:")
    for url in test_urls:
        normalized = normalize_url(url)
        fingerprint = url_fingerprint(url)[:8]
        print(f"  {url}")
        print(f"    → {normalized} [{fingerprint}]")
    
    # Test deduplication
    test_endpoints = [
        {'url': '/api/users?id=1'},
        {'url': '/api/users?id=2'},  # Duplicate structure
        {'url': '/api/posts'},
    ]
    
    deduplicated = deduplicate_endpoints(test_endpoints)
    print(f"\nDeduplication: {len(test_endpoints)} → {len(deduplicated)} endpoints")

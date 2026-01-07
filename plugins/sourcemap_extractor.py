"""
Sourcemap Extractor Plugin for Endpointo

This plugin extracts sourcemap URLs from JavaScript files
and adds sourcemap metadata to endpoints.
"""

import re
import json


def extract_sourcemap_url(content):
    """
    Extract sourcemap URL from JavaScript content.
    
    Args:
        content (str): JavaScript source code
        
    Returns:
        str: Sourcemap URL or None
    """
    # Match sourceMappingURL comment
    patterns = [
        r'//# sourceMappingURL=(.+)$',
        r'/\*# sourceMappingURL=(.+)\*/',
    ]
    
    for pattern in patterns:
        match = re.search(pattern, content, re.MULTILINE)
        if match:
            return match.group(1).strip()
    
    return None


def parse_sourcemap(sourcemap_content):
    """
    Parse sourcemap JSON content.
    
    Args:
        sourcemap_content (str): Sourcemap JSON string
        
    Returns:
        dict: Parsed sourcemap or None
    """
    try:
        return json.loads(sourcemap_content)
    except json.JSONDecodeError:
        return None


def transform_endpoint(endpoint):
    """
    Add sourcemap metadata to endpoint.
    
    Args:
        endpoint (dict): Endpoint data
        
    Returns:
        dict: Endpoint with sourcemap metadata
    """
    source = endpoint.get('source', '')
    
    # Mark if source likely has a sourcemap
    if source.endswith('.min.js') or source.endswith('.bundle.js'):
        if 'metadata' not in endpoint:
            endpoint['metadata'] = {}
        endpoint['metadata']['likely_has_sourcemap'] = True
        endpoint['metadata']['original_source'] = source
    
    return endpoint


def parse_custom(content, source):
    """
    Extract endpoints from sourcemap sources.
    
    Args:
        content (str): File content
        source (str): Source file path
        
    Returns:
        list: List of endpoints found in sourcemap
    """
    endpoints = []
    
    # Check if this is a sourcemap file
    if source.endswith('.map'):
        sourcemap = parse_sourcemap(content)
        if sourcemap and 'sources' in sourcemap:
            # Extract original source files
            for original_source in sourcemap['sources']:
                endpoints.append({
                    'url': original_source,
                    'endpoint_type': 'unknown',
                    'source': source,
                    'metadata': {
                        'from_sourcemap': True,
                        'original_file': original_source
                    }
                })
    
    return endpoints


# Example usage
if __name__ == '__main__':
    # Test sourcemap URL extraction
    test_js = """
    !function(){console.log("minified")}();
    //# sourceMappingURL=bundle.js.map
    """
    
    url = extract_sourcemap_url(test_js)
    print(f"Extracted sourcemap URL: {url}")
    
    # Test endpoint transformation
    test_endpoint = {
        'url': '/api/users',
        'source': 'app.min.js'
    }
    
    result = transform_endpoint(test_endpoint)
    print(f"Transformed endpoint: {result}")

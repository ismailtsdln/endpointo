# Endpointo Plugin Development Guide

This directory contains Python plugins for extending Endpointo's functionality.

## Overview

Endpointo supports Python plugins that can:
- Filter endpoints based on custom logic  
- Transform endpoint data
- Add custom parsing rules
- Implement framework-specific analyzers

## Plugin Structure

A basic plugin is a Python file with specific functions:

```python
def filter_endpoint(endpoint: dict) -> bool:
    """
    Filter an endpoint based on custom logic.
    
    Args:
        endpoint: Dictionary with keys: url, method, endpoint_type, source
        
    Returns:
        True if endpoint should be included, False otherwise
    """
    return True

def transform_endpoint(endpoint: dict) -> dict:
    """
    Transform endpoint data.
    
    Args:
        endpoint: Endpoint dictionary
        
    Returns:
        Modified endpoint dictionary
    """
    return endpoint

def parse_custom(content: str, source: str) -> list:
    """
    Custom parsing logic for extracting endpoints.
    
    Args:
        content: JavaScript/HTML content to parse
        source: Source file path or URL
        
    Returns:
        List of endpoint dictionaries
    """
    return []
```

## Example Plugins

### regex_filter.py

Custom regex-based endpoint filtering:

```python
import re

def filter_endpoint(endpoint):
    # Only include API endpoints
    api_pattern = re.compile(r'/api/v\d+/')
    return api_pattern.search(endpoint['url']) is not None
```

### sourcemap_extractor.py

Extract and parse sourcemaps:

```python
import json
import re

def parse_sourcemap_url(content):
    """Extract sourcemap URL from JavaScript"""
    match = re.search(r'//# sourceMappingURL=(.+)$', content, re.MULTILINE)
    return match.group(1) if match else None

def transform_endpoint(endpoint):
    """Add sourcemap information"""
    if endpoint.get('source'):
        # Add sourcemap metadata
        endpoint['has_sourcemap'] = True
    return endpoint
```

### dedup.py

Advanced URL deduplication:

```python
from urllib.parse import urlparse, parse_qs

def normalize_url(url):
    """Normalize URL for deduplication"""
    parsed = urlparse(url)
    
    # Sort query parameters
    query_params = parse_qs(parsed.query)
    sorted_params = sorted(query_params.items())
    
    # Rebuild URL without parameter values
    param_keys = '&'.join([f"{k}=" for k, _ in sorted_params])
    
    return f"{parsed.scheme}://{parsed.netloc}{parsed.path}?{param_keys}"

def transform_endpoint(endpoint):
    """Add normalized URL for deduplication"""
    endpoint['normalized_url'] = normalize_url(endpoint['url'])
    return endpoint
```

## Using Plugins

### Command Line

```bash
# Use a single plugin
endpointo scan -u https://example.com --plugin ./plugins/regex_filter.py

# Chain multiple plugins (when supported)
endpointo scan -u https://example.com \
    --plugin ./plugins/regex_filter.py \
    --plugin ./plugins/dedup.py
```

### Plugin API Reference

#### Endpoint Structure

```python
{
    "url": "/api/v1/users",
    "method": "GET",  # Optional
    "endpoint_type": "rest",  # rest, graphql, websocket, unknown
    "source": "https://example.com/app.js",  # Optional
    "line": 42,  # Optional
    "params": ["id", "name"],  # Optional
    "metadata": {}  # Optional
}
```

#### Plugin Functions

All plugin functions are optional. Implement only what you need:

- **filter_endpoint(endpoint: dict) -> bool**: Return True to include endpoint
- **transform_endpoint(endpoint: dict) -> dict**: Modify and return endpoint
- **parse_custom(content: str, source: str) -> list**: Custom parsing logic

## Best Practices

1. **Error Handling**: Always handle errors gracefully
   ```python
   def filter_endpoint(endpoint):
       try:
           return 'api' in endpoint.get('url', '')
       except Exception as e:
           print(f"Filter error: {e}")
           return True  # Default to including endpoint
   ```

2. **Performance**: Keep plugins fast (they run on every endpoint)
   ```python
   # Compile regex once, not per endpoint
   API_PATTERN = re.compile(r'/api/')
   
   def filter_endpoint(endpoint):
       return API_PATTERN.search(endpoint['url']) is not None
   ```

3. **Documentation**: Add docstrings and comments
   ```python
   def filter_endpoint(endpoint):
       """Only include endpoints containing '/admin/'"""
       return '/admin/' in endpoint['url']
   ```

4. **Dependencies**: Minimize external dependencies
   - Prefer standard library when possible
   - Document required packages in plugin header

## Framework-Specific Plugins

### React Plugin

```python
def parse_custom(content, source):
    """Extract React Router endpoints"""
    import re
    
    # Match <Route path="/..." />
    routes = re.findall(r'<Route\s+path="([^"]+)"', content)
    
    endpoints = []
    for route in routes:
        endpoints.append({
            'url': route,
            'endpoint_type': 'rest',
            'source': source,
            'metadata': {'framework': 'react'}
        })
    
    return endpoints
```

### Vue Plugin

```python
def parse_custom(content, source):
    """Extract Vue Router endpoints"""
    import re
    
    # Match { path: '/...' }
    paths = re.findall(r"path:\s*['\"]([^'\"]+)['\"]", content)
    
    return [
        {
            'url': path,
            'endpoint_type': 'rest',
            'source': source,
            'metadata': {'framework': 'vue'}
        }
        for path in paths
    ]
```

## Testing Plugins

Create a test script:

```python
# test_plugin.py
from your_plugin import filter_endpoint, transform_endpoint

def test_filter():
    endpoint = {'url': '/api/v1/users', 'endpoint_type': 'rest'}
    assert filter_endpoint(endpoint) == True

def test_transform():
    endpoint = {'url': '/api/users'}
    result = transform_endpoint(endpoint)
    assert 'normalized_url' in result

if __name__ == '__main__':
    test_filter()
    test_transform()
    print("All tests passed!")
```

Run tests:
```bash
python test_plugin.py
```

## Contributing Plugins

To contribute a plugin:

1. Create your plugin in `plugins/`
2. Add documentation and examples
3. Test thoroughly
4. Submit a PR with description

## Troubleshooting

### Plugin Not Loading

- Check file permissions
- Verify Python syntax
- Ensure file has `.py` extension

### Plugin Errors

Enable verbose logging:
```bash
RUST_LOG=debug endpointo scan -u https://example.com --plugin ./my_plugin.py
```

## Future Features

Planned plugin capabilities:
- [ ] Async plugin support
- [ ] Plugin configuration files
- [ ] Plugin chaining and composition
- [ ] Built-in plugin marketplace

## Support

For plugin development questions:
- 📚 [Documentation](https://github.com/ismailtsdln/endpointo/docs)
- 💬 [Discussions](https://github.com/ismailtsdln/endpointo/discussions)
- 🐛 [Issues](https://github.com/ismailtsdln/endpointo/issues)

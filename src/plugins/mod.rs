#[cfg(feature = "python-plugins")]
use pyo3::prelude::*;
#[cfg(feature = "python-plugins")]
use pyo3::types::PyDict;

#[cfg(feature = "python-plugins")]
use crate::error::Error;
use crate::error::Result;
use crate::types::Endpoint;
use std::path::Path;
use tracing::info;

#[cfg(feature = "python-plugins")]
use tracing::error;

/// Plugin manager handles loading and executing Python plugins
pub struct PluginManager {
    #[cfg(feature = "python-plugins")]
    plugins: Vec<Py<PyAny>>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "python-plugins")]
            plugins: Vec::new(),
        }
    }

    /// Load a Python plugin from file
    pub fn load_plugin(&mut self, _path: &Path) -> Result<()> {
        #[cfg(feature = "python-plugins")]
        {
            info!("Loading plugin from: {}", _path.display());

            let code = std::fs::read_to_string(_path)?;
            let filename = _path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("plugin.py");

            Python::with_gil(|py| -> Result<()> {
                let module = PyModule::from_code_bound(py, &code, filename, filename)
                    .map_err(|e| Error::PluginError(format!("Failed to parse plugin: {}", e)))?;

                self.plugins.push(module.into());
                Ok(())
            })?;
        }

        #[cfg(not(feature = "python-plugins"))]
        {
            info!(
                "Python plugins are disabled. Skipping plugin: {}",
                _path.display()
            );
        }

        Ok(())
    }

    /// Execute filter_endpoint on all plugins
    pub fn filter_endpoint(&self, _endpoint: &Endpoint) -> bool {
        #[cfg(feature = "python-plugins")]
        {
            return Python::with_gil(|py| {
                for plugin in &self.plugins {
                    let plugin = plugin.bind(py);
                    if let Ok(filter_fn) = plugin.getattr("filter_endpoint") {
                        let ep_dict = self.endpoint_to_dict(py, _endpoint);
                        match filter_fn.call1((ep_dict,)) {
                            Ok(result) => {
                                if let Ok(keep) = result.extract::<bool>() {
                                    if !keep {
                                        return false;
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Plugin filter_endpoint error: {}", e);
                            }
                        }
                    }
                }
                true
            });
        }

        #[cfg(not(feature = "python-plugins"))]
        true
    }

    /// Execute transform_endpoint on all plugins
    pub fn transform_endpoint(&self, mut endpoint: Endpoint) -> Endpoint {
        #[cfg(feature = "python-plugins")]
        {
            Python::with_gil(|py| {
                for plugin in &self.plugins {
                    let plugin = plugin.bind(py);
                    if let Ok(transform_fn) = plugin.getattr("transform_endpoint") {
                        let ep_dict = self.endpoint_to_dict(py, &endpoint);
                        match transform_fn.call1((ep_dict,)) {
                            Ok(result) => {
                                if let Ok(new_ep_dict) = result.downcast_bound::<PyDict>() {
                                    if let Ok(new_ep) = self.dict_to_endpoint(new_ep_dict) {
                                        endpoint = new_ep;
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Plugin transform_endpoint error: {}", e);
                            }
                        }
                    }
                }
            });
        }
        endpoint
    }

    /// Convert Endpoint to Python dictionary
    #[cfg(feature = "python-plugins")]
    fn endpoint_to_dict<'py>(&self, py: Python<'py>, endpoint: &Endpoint) -> Bound<'py, PyDict> {
        let dict = PyDict::new_bound(py);
        let _ = dict.set_item("url", &endpoint.url).ok();
        let _ = dict.set_item("method", &endpoint.method).ok();
        let _ = dict
            .set_item(
                "endpoint_type",
                format!("{:?}", endpoint.endpoint_type).to_lowercase(),
            )
            .ok();
        let _ = dict.set_item("source", &endpoint.source).ok();
        let _ = dict.set_item("line", endpoint.line).ok();
        let _ = dict.set_item("params", &endpoint.params).ok();
        dict
    }

    /// Convert Python dictionary to Endpoint
    #[cfg(feature = "python-plugins")]
    fn dict_to_endpoint(&self, dict: &Bound<'_, PyDict>) -> Result<Endpoint> {
        let url: String = dict
            .get_item("url")
            .and_then(|p| p.map(|item| item.extract::<String>().ok()).flatten())
            .ok_or_else(|| Error::PluginError("Missing url in transformed endpoint".to_string()))?;

        let endpoint_type_str: String = dict
            .get_item("endpoint_type")
            .and_then(|p| p.map(|item| item.extract::<String>().ok()).flatten())
            .unwrap_or_else(|| "unknown".to_string());

        let endpoint_type = match endpoint_type_str.as_str() {
            "rest" => crate::types::EndpointType::Rest,
            "graphql" => crate::types::EndpointType::GraphQL,
            "websocket" => crate::types::EndpointType::WebSocket,
            _ => crate::types::EndpointType::Unknown,
        };

        let mut endpoint = Endpoint::new(url, endpoint_type);

        if let Some(item) = dict.get_item("method").and_then(|i| i.ok()) {
            if let Ok(method) = item.extract::<String>() {
                endpoint = endpoint.with_method(method);
            }
        }

        if let Some(item) = dict.get_item("source").and_then(|i| i.ok()) {
            if let Ok(source) = item.extract::<String>() {
                endpoint = endpoint.with_source(source);
            }
        }

        if let Some(item) = dict.get_item("line").and_then(|i| i.ok()) {
            if let Ok(line) = item.extract::<usize>() {
                endpoint = endpoint.with_line(line);
            }
        }

        if let Some(item) = dict.get_item("params").and_then(|i| i.ok()) {
            if let Ok(params) = item.extract::<Vec<String>>() {
                endpoint = endpoint.with_params(params);
            }
        }

        Ok(endpoint)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new()
    }
}

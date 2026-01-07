pub use crate::cli::OutputFormat;
use crate::error::{Error, Result};
use crate::types::Endpoint;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Write scan results to output
pub fn write_results(
    endpoints: &[Endpoint],
    output_path: Option<&Path>,
    format: OutputFormat,
) -> Result<()> {
    let output = match format {
        OutputFormat::Json => serialize_json(endpoints)?,
        OutputFormat::Yaml => serialize_yaml(endpoints)?,
        OutputFormat::Xml => serialize_xml(endpoints)?,
        OutputFormat::Html => serialize_html(endpoints)?,
    };

    if let Some(path) = output_path {
        let mut file = File::create(path)?;
        file.write_all(output.as_bytes())?;
    } else {
        println!("{}", output);
    }

    Ok(())
}

/// Serialize to JSON
fn serialize_json(endpoints: &[Endpoint]) -> Result<String> {
    serde_json::to_string_pretty(endpoints).map_err(Error::from)
}

/// Serialize to YAML
fn serialize_yaml(endpoints: &[Endpoint]) -> Result<String> {
    serde_yaml::to_string(endpoints).map_err(Error::from)
}

/// Serialize to XML
fn serialize_xml(endpoints: &[Endpoint]) -> Result<String> {
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    xml.push_str("\n<endpoints>");

    for endpoint in endpoints {
        xml.push_str("\n  <endpoint>");
        xml.push_str(&format!("\n    <url>{}</url>", escape_xml(&endpoint.url)));

        if let Some(method) = &endpoint.method {
            xml.push_str(&format!("\n    <method>{}</method>", escape_xml(method)));
        }

        xml.push_str(&format!("\n    <type>{:?}</type>", endpoint.endpoint_type));

        if let Some(source) = &endpoint.source {
            xml.push_str(&format!("\n    <source>{}</source>", escape_xml(source)));
        }

        xml.push_str("\n  </endpoint>");
    }

    xml.push_str("\n</endpoints>");
    Ok(xml)
}

/// Serialize to HTML report
fn serialize_html(_endpoints: &[Endpoint]) -> Result<String> {
    // TODO: Implement HTML template with Askama
    let html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Endpointo Scan Results</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        h1 { color: #333; border-bottom: 2px solid #4CAF50; padding-bottom: 10px; }
        table { width: 100%; border-collapse: collapse; margin-top: 20px; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
        th { background: #4CAF50; color: white; }
        tr:hover { background: #f5f5f5; }
        .badge { padding: 4px 8px; border-radius: 4px; font-size: 12px; }
        .rest { background: #2196F3; color: white; }
        .graphql { background: #E91E63; color: white; }
        .websocket { background: #FF9800; color: white; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔍 Endpointo Scan Results</h1>
        <p>HTML report generation in progress...</p>
    </div>
</body>
</html>
    "#;

    Ok(html.to_string())
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

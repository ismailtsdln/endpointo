pub use crate::cli::OutputFormat;
use crate::error::{Error, Result};
use crate::types::Endpoint;
use colored::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Write scan results to output
pub fn write_results(
    endpoints: &[Endpoint],
    output_path: Option<&Path>,
    format: OutputFormat,
) -> Result<()> {
    if let Some(path) = output_path {
        let output = match format {
            OutputFormat::Json => serialize_json(endpoints)?,
            OutputFormat::Yaml => serialize_yaml(endpoints)?,
            OutputFormat::Xml => serialize_xml(endpoints)?,
            OutputFormat::Html => serialize_html(endpoints)?,
        };
        let mut file = File::create(path)?;
        file.write_all(output.as_bytes())?;
    } else {
        display_to_terminal(endpoints);
    }

    Ok(())
}

/// Display endpoints to terminal with colors and formatting
fn display_to_terminal(endpoints: &[Endpoint]) {
    println!(
        "\n{}",
        "🔍 Discovered Endpoints".bold().bright_white().on_blue()
    );
    println!("{}", "─".repeat(80).dimmed());

    for ep in endpoints {
        let method = ep.method.as_deref().unwrap_or("GET").to_uppercase();
        let method_colored = match method.as_str() {
            "GET" => method.green(),
            "POST" => method.blue(),
            "PUT" => method.yellow(),
            "DELETE" => method.red(),
            _ => method.normal(),
        };

        let type_badge = match ep.endpoint_type {
            crate::types::EndpointType::Rest => " REST ".black().on_bright_blue(),
            crate::types::EndpointType::GraphQL => " GQL  ".black().on_bright_magenta(),
            crate::types::EndpointType::WebSocket => "  WS  ".black().on_bright_green(),
            _ => " UNK  ".black().on_white(),
        };

        println!(
            "{} {:<7} {} {}",
            type_badge,
            method_colored.bold(),
            ep.url.bright_white(),
            format!("({})", ep.source.as_deref().unwrap_or("-")).dimmed()
        );
    }

    println!("{}", "─".repeat(80).dimmed());
    println!(
        "{} {}",
        "Total endpoints:".bold(),
        endpoints.len().to_string().bright_green()
    );
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
fn serialize_html(endpoints: &[Endpoint]) -> Result<String> {
    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Endpointo Report</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 20px; background-color: #f5f5f5; }
        h1 { color: #333; }
        .container { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        table { width: 100%; border-collapse: collapse; margin-top: 20px; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
        th { background-color: #f8f9fa; color: #333; cursor: pointer; }
        tr:hover { background-color: #f1f1f1; }
        .badge { padding: 4px 8px; border-radius: 4px; font-size: 0.85em; font-weight: bold; }
        .badge-rest { background: #e3f2fd; color: #1976d2; }
        .badge-graphql { background: #f3e5f5; color: #7b1fa2; }
        .badge-websocket { background: #e8f5e9; color: #388e3c; }
        .badge-unknown { background: #eeeeee; color: #616161; }
        #search { padding: 10px; width: 300px; margin-bottom: 10px; border: 1px solid #ccc; border-radius: 4px; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🔍 Endpointo Scan Report</h1>
        <input type="text" id="search" onkeyup="filterTable()" placeholder="Search URLs, methods, sources...">
        <table id="resultsTable">
            <thead>
                <tr>
                    <th onclick="sortTable(0)">URL</th>
                    <th onclick="sortTable(1)">Type</th>
                    <th onclick="sortTable(2)">Method</th>
                    <th onclick="sortTable(3)">Source</th>
                </tr>
            </thead>
            <tbody>
"#,
    );

    for ep in endpoints {
        let badge_class = match ep.endpoint_type {
            crate::types::EndpointType::Rest => "badge-rest",
            crate::types::EndpointType::GraphQL => "badge-graphql",
            crate::types::EndpointType::WebSocket => "badge-websocket",
            _ => "badge-unknown",
        };

        html.push_str(&format!(
            r#"                <tr>
                    <td>{}</td>
                    <td><span class="badge {}">{}</span></td>
                    <td>{}</td>
                    <td>{}:{}</td>
                </tr>
"#,
            escape_xml(&ep.url),
            badge_class,
            format!("{:?}", ep.endpoint_type),
            ep.method.as_deref().unwrap_or("-"),
            ep.source.as_deref().unwrap_or("-"),
            ep.line
                .map(|l: usize| l.to_string())
                .unwrap_or_else(|| "-".to_string())
        ));
    }

    html.push_str(
        r#"            </tbody>
        </table>
    </div>

    <script>
        function filterTable() {
            var input, filter, table, tr, td, i, j, found;
            input = document.getElementById("search");
            filter = input.value.toUpperCase();
            table = document.getElementById("resultsTable");
            tr = table.getElementsByTagName("tr");
            for (i = 1; i < tr.length; i++) {
                found = false;
                td = tr[i].getElementsByTagName("td");
                for (j = 0; j < td.length; j++) {
                    if (td[j].textContent.toUpperCase().indexOf(filter) > -1) {
                        found = true;
                        break;
                    }
                }
                tr[i].style.display = found ? "" : "none";
            }
        }

        function sortTable(n) {
            var table, rows, switching, i, x, y, shouldSwitch, dir, switchcount = 0;
            table = document.getElementById("resultsTable");
            switching = true;
            dir = "asc";
            while (switching) {
                switching = false;
                rows = table.rows;
                for (i = 1; i < (rows.length - 1); i++) {
                    shouldSwitch = false;
                    x = rows[i].getElementsByTagName("TD")[n];
                    y = rows[i + 1].getElementsByTagName("TD")[n];
                    if (dir == "asc") {
                        if (x.innerHTML.toLowerCase() > y.innerHTML.toLowerCase()) { shouldSwitch = true; break; }
                    } else if (dir == "desc") {
                        if (x.innerHTML.toLowerCase() < y.innerHTML.toLowerCase()) { shouldSwitch = true; break; }
                    }
                }
                if (shouldSwitch) {
                    rows[i].parentNode.insertBefore(rows[i + 1], rows[i]);
                    switching = true;
                    switchcount ++;
                } else if (switchcount == 0 && dir == "asc") {
                    dir = "desc";
                    switching = true;
                }
            }
        }
    </script>
</body>
</html>"#,
    );

    Ok(html)
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

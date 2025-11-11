# 🦀 Rust (via Cargo)

This document outlines all usage examples for Rust development and production.

## A. As a CLI Tool

### Installation
```bash
# Install from local source
cargo install --path crates/datadown

# Install from crates.io (after publishing)
cargo install datadown
```

### Command-Line Usage
```bash
# From a file → stdout (JSON default)
datadown input.md

# From a file → specific format
datadown input.md --format yaml
datadown input.md -f toml
datadown input.md -f xml

# From stdin
echo "# Hello World" | datadown -
cat README.md | datadown - --format json

# To output file
datadown input.md --format yaml --out output.yaml
datadown input.md -f json -o output.json

# Show version
datadown --version

# Show help
datadown --help
```

### Development (without installing):
```bash
# Run directly from source
cargo run --package datadown -- input.md --format json
echo "# Test" | cargo run --package datadown -- - -f yaml
```

---

## B. As a Rust Library

### Add to your project
```toml
# Cargo.toml
[dependencies]
datadown = "0.1.0"

# With all format features
datadown = { version = "0.1.0", features = ["yaml", "toml", "xml"] }

# Only specific formats
datadown = { version = "0.1.0", features = ["yaml"] }
```

### Library Usage - High-Level API
```rust
use datadown::{convert_str, OutputFormat};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let markdown = "# Hello World\n\nThis is **bold** text.";

    // Convert to JSON
    let json = convert_str(markdown, OutputFormat::Json)?;
    println!("{}", json);

    // Convert to YAML
    let yaml = convert_str(markdown, OutputFormat::Yaml)?;
    println!("{}", yaml);

    // Convert to TOML
    let toml = convert_str(markdown, OutputFormat::Toml)?;
    println!("{}", toml);

    // Convert to XML
    let xml = convert_str(markdown, OutputFormat::Xml)?;
    println!("{}", xml);

    Ok(())
}
```

### Library Usage - Low-Level API (Working with AST)
```rust
use datadown::{parse_markdown, Node};

fn main() {
    let markdown = "# Title\n\nParagraph with *italic*.";

    // Parse to AST
    let ast = parse_markdown(markdown);

    // Work with the AST directly
    match ast {
        Node::Document { children } => {
            println!("Document has {} top-level nodes", children.len());

            for child in children {
                match child {
                    Node::Heading { level, text } => {
                        println!("Found heading level {}: {}", level, text);
                    }
                    Node::Paragraph { children } => {
                        println!("Found paragraph with {} elements", children.len());
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }

    // Serialize AST to any format using serde
    let json = serde_json::to_string_pretty(&ast).unwrap();
    println!("{}", json);
}

Using OutputFormat from String

use datadown::{convert_str, OutputFormat};

fn convert_with_string_format(md: &str, format: &str) -> String {
    let fmt = OutputFormat::from_str(format)
        .expect("Invalid format");

    convert_str(md, fmt).expect("Conversion failed")
}

// Usage
let json = convert_with_string_format("# Test", "json");
let yaml = convert_with_string_format("# Test", "yaml");
let toml = convert_with_string_format("# Test", "toml");
```
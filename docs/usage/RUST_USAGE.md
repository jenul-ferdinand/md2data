# 🦀 Rust (via Cargo)

## A. As a CLI Tool

### Installation methods
```bash
# Install from local source
cargo install --path crates/md2data

# Install from crates.io (after publishing)
cargo install md2data
```

### Command-Line usage
```bash
# From a file → stdout (JSON default)
md2data input.md

# From a file → specific format
md2data input.md --format yaml
md2data input.md -f toml
md2data input.md -f xml

# From stdin
echo "# Hello World" | md2data -
cat README.md | md2data - --format json

# To output file
md2data input.md --format yaml --out output.yaml
md2data input.md -f json -o output.json

# Show version
md2data --version

# Show help
md2data --help
```

### Development (without installing)
```bash
# Run directly from source
cargo run --package md2data -- input.md --format json
echo "# Test" | cargo run --package md2data -- - -f yaml
```

---

## B. As a Rust Library

### Add to your project
```toml
# Cargo.toml
[dependencies]
md2data = "0.1.0"

# With all format features
md2data = { version = "0.1.0", features = ["yaml", "toml", "xml"] }

# Only specific formats
md2data = { version = "0.1.0", features = ["yaml"] }
```

### Library Usage - High-Level API
```rust
use md2data::{convert_str, OutputFormat};

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
use md2data::{parse_markdown, Node};

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

use md2data::{convert_str, OutputFormat};

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
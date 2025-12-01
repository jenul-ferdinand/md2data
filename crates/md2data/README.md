# md2data for Rust 📦🦀
> Markdown → JSON/YAML/TOML/XML. Faster with Rust.

## Installation
Add to your `Cargo.toml`:
```toml
[dependencies]
md2data = "0.1"
```

Or use cargo add:
```bash
cargo add md2data
```

### Features
By default, only JSON support is enabled. Enable other formats as needed:
```toml
[dependencies]
md2data = { version = "0.1", features = ["yaml", "toml", "xml"] }
```

## Usage
```rust
use md2data::{convert_str, OutputFormat, ParsingMode};

fn main() {
    let markdown = r#"# Hello World

This is a **markdown** document."#;

    // Convert to different formats
    let json = convert_str(markdown, OutputFormat::Json, ParsingMode::Minified).unwrap();
    let yaml = convert_str(markdown, OutputFormat::Yaml, ParsingMode::Minified).unwrap();
    let toml = convert_str(markdown, OutputFormat::Toml, ParsingMode::Minified).unwrap();
    let xml = convert_str(markdown, OutputFormat::Xml, ParsingMode::Minified).unwrap();

    println!("{}", json);
}
```

### Parsing Modes
- `ParsingMode::Minified` (default): Compact representation
- `ParsingMode::Document`: Full document structure with detailed node information

## Developers: Build process
The build process can be initiated with `cargo build`.

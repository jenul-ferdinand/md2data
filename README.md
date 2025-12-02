# md2data: Markdown &rarr; Structured Data

<p align="center">
  <img width="300px" alt="MD2Data Logo" src="docs/assets/logo.png" />
</p>

A high-performance tool that parses Markdown documents and converts them into structured data formats i.e., JSON, YAML, XML, or TOML. Built with Rust for maximum speed and reliability, with bindings for Node.js and Python. Available as both a Rust library and CLI tool.

## How It Works

md2data works in three steps.

```
Markdown (input)
   ↓
Parser (Markdown → AST)
   ↓
Serializer (AST → JSON/YAML/TOML/XML)
```

Markdown is parsed into an [Abstract Syntax Tree (AST)](https://en.wikipedia.org/wiki/Abstract_syntax_tree) using [`pulldown-cmark`](https://github.com/pulldown-cmark/pulldown-cmark), which is also a Rust-based CommonMark compliant parser. The AST is simply converted to your desired output format using [`serde`](https://github.com/serde-rs/serde).

### Architecture

The core parsing and serialization logic can be found in [`crates/md2data`](/crates/md2data/).

The bindings for Python and Node.js are in [`bindings/python`](/bindings/python/) ([PyO3](https://github.com/PyO3/pyo3) & [maturin](https://github.com/PyO3/maturin)) and [`bindings/node`](/bindings/node/) ([napi-rs](https://napi.rs/)).

### Features

- **Multiple output formats**: Convert Markdown to JSON, YAML, TOML, or XML.
- **High performance**: Written in Rust with zero-cost abstractions.
- **Python and Node.js support**: Bindings available for Python and Node.js too.
- **Structured Output**: Generates a clean data representation of your Markdown.


## Getting Started

### Installation

📦 Cargo
```bash
cargo install md2data
```

🦀 Manual installation for Rust
```bash
git clone https://github.com/yourusername/md2data.git
cd md2data
cargo build --release
```

⚡ Node.js
```bash
npm install md2data
```

🐍 Python
```bash
pip install md2data
```

### Usage

<details>
<summary><strong>As a rust library</strong></summary>

```rust
use md2data::{convert_str, OutputFormat, ParsingMode};

let markdown = r#"# Hello World

This is a **markdown** document."#;

let json = convert_str(markdown, OutputFormat::Json, ParsingMode::Minified).unwrap();
let yaml = convert_str(markdown, OutputFormat::Yaml, ParsingMode::Minified).unwrap();
```

</details>

<details>
<summary><strong>In the command line</strong></summary>

```bash
# Convert to JSON (default)
md2data input.md

# Specify output format
md2data input.md --format yaml
md2data input.md --format toml
md2data input.md --format xml

# Read from stdin
echo "# Hello World" | md2data - --format json

# Output to file
md2data input.md --format json -o output.json
```

</details>

<details>
<summary><strong>In a node.js script</strong></summary>

```javascript
const { convert } = require('md2data');

const markdown = `
# Hello World

This is a **markdown** document.
`;

const json = convert(markdown, 'json');
const yaml = convert(markdown, 'yaml');
const toml = convert(markdown, 'toml');
const xml = convert(markdown, 'xml');

console.log(json);
```

</details>

<details>
<summary><strong>In a python script</strong></summary>

```python
from md2data import convert

markdown = """
# Hello World

This is a **markdown** document.
"""

json_output = convert(markdown, 'json')
yaml_output = convert(markdown, 'yaml')
toml_output = convert(markdown, 'toml')
xml_output = convert(markdown, 'xml')

print(json_output)
```

</details>

### Output Examples
See [docs/examples](https://github.com/jenul-ferdinand/md2data/tree/main/docs/examples) for example outputs.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/md2data.git
cd md2data

# Build all components
cargo build

# Run tests
cargo test

# Build Node.js binding
cd bindings/node
npm install
npm run build

# Build Python binding
cd bindings/python
maturin develop
```

## License

MIT License - see LICENSE file for details

## Acknowledgments

- Built with [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) for Markdown parsing
- Inspired by existing tools like [md2json](https://www.npmjs.com/package/md2json) and [markdown-to-json](https://www.npmjs.com/package/markdown-to-json)
- Designed to provide better multi-format support and cross-language compatibility
- Classic EmojiOne (now [JoyPixels](https://joypixels.com/)) emojis used for the logo design.

---

Made with Rust 🦀

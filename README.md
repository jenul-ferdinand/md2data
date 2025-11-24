# Markdown &rarr; Structured Data

## Description
A high-performance tool that parses Markdown documents and converts them into structured data formats i.e., JSON/YAML/XML/TOML. Built with Rust for maximum speed and reliability, bindings for Node.js, Python provided with a standalone CLI.

## Features

- **Multiple Output Formats**: Convert Markdown to JSON, YAML, TOML, or XML
- **Cross-Platform**: Available as a Rust CLI, Node.js package, and Python package
- **High Performance**: Written in Rust with zero-cost abstractions
- **Simple API**: Easy-to-use interface across Python, Node.js and Rust
- **Structured AST**: Generates a clean Abstract Syntax Tree representation of your Markdown

## Installation

### CLI (Rust)

```bash
# Build from source
git clone https://github.com/yourusername/md2data.git
cd md2data
cargo build --release
```

### Node.js

```bash
npm install md2data
```

### Python

```bash
pip install md2data
```

## Usage

### Command Line

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

### Node.js

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

### Python

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

## Example



## How It Works

md2data uses a three-layer architecture:

```
Markdown (input)
   ↓
Parser (Markdown → AST)
   ↓
Serializer (AST → JSON/YAML/TOML/XML)
```

1. **Markdown Parsing**: Uses `pulldown-cmark`, a fast CommonMark-compliant parser
2. **AST Generation**: Builds a structured Abstract Syntax Tree with typed nodes
3. **Format Serialization**: Converts the AST to your desired output format using `serde`

### Architecture

- **Core Library** (`md2data`): Rust library with parsing and serialization logic
- **Node.js Binding**: Native addon built with `napi-rs`
- **Python Binding**: Native extension built with `PyO3` and `maturin`

## Technology Stack

| Component | Technology |
|-----------|------------|
| Language | Rust 1.91+ |
| Markdown Parser | `pulldown-cmark` 0.10 |
| Serialization | `serde` 1.x |
| JSON Output | `serde_json` 1.x |
| YAML Output | `serde_yaml` 0.9 |
| TOML Output | `toml` 0.8 |
| XML Output | `quick-xml` 0.36 |
| CLI Framework | `clap` 4.x |
| Node.js Binding | `napi-rs` 2.x |
| Python Binding | `PyO3` + `maturin` |
| Error Handling | `thiserror` 1.x |

## Roadmap: Markdown Feature Support

### Currently Supported ✅

- [x] Headings (H1-H6)
- [x] Paragraphs
- [x] Text content
- [x] Code blocks
- [x] Lists (ordered and unordered)
- [x] Nested lists
- [x] List items
- [x] Inline code
- [x] Line breaks (soft and hard)

### Inline Formatting (Text Preserved, Formatting Stripped) ⚠️

- [x] Bold/Strong text (text content preserved)
- [x] Italic/Emphasis text (text content preserved)
- [x] Strikethrough (text content preserved)

### Planned Features 🚧

- [ ] Links (inline and reference-style)
- [ ] Images
- [ ] Block quotes
- [ ] Horizontal rules
- [ ] Tables
- [ ] Task lists (GitHub Flavored Markdown)
- [ ] Footnotes
- [ ] Code block language detection
- [ ] Preserve inline formatting in AST (not just text)
- [ ] HTML passthrough
- [ ] Definition lists

### Future Enhancements 💡

- [ ] Custom output format plugins
- [ ] Configuration file support
- [ ] Streaming API for large documents
- [ ] Incremental parsing
- [ ] Source position tracking
- [ ] Schema validation
- [ ] Pretty-printing options

## AST Node Types

The parser generates the following node types:

- **Document**: Root container with children
- **Heading**: Headers with level (1-6) and text content
- **Paragraph**: Paragraph container with child nodes
- **Text**: Plain text content
- **CodeBlock**: Code blocks with optional language info
- **List**: Ordered or unordered lists with items
- **ListItem**: Individual list item with children

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

---

Made with Rust 🦀

# Datadown for Python 📦🐍 
> Markdown → JSON/YAML/TOML/XML. Faster with Rust.

## Installation
Install with pip
```bash
pip install datadown
```

## Usage
```python
import datadown

md = """# Hello World

This is a **markdown** document."""

json = datadown.convert(md, "json")
yaml = datadown.convert(md, "yaml")
toml = datadown.convert(md, "toml")
xml = datadown.convert(md, "xml")
```

## Developers: Building the wheel
The build process can be initiated with `uv build`. A dist folder will be created.
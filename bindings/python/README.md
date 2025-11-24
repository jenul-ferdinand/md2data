# md2data for Python 📦🐍 
> Markdown → JSON/YAML/TOML/XML. Faster with Rust.

## Installation
Install with pip
```bash
pip install md2data
```

## Usage
```python
import md2daa

md = """# Hello World

This is a **markdown** document."""

json = md2data.convert(md, "json")
yaml = md2data.convert(md, "yaml")
toml = md2data.convert(md, "toml")
xml = md2data.convert(md, "xml")
```

## Developers: Building the wheel
The build process can be initiated with `uv build`. A dist folder will be created.
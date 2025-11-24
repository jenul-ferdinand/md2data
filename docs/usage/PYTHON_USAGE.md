# 🐍 Python

This document outlines all usage examples for Python development and production.

## Installation
```bash
# Install from local wheel
pip install bindings/python/dist/md2data-0.1.0-cp314-cp314-win_amd64.whl

# Install from PyPI
pip install md2data

# Install with uv
uv pip install md2data
# Add to pyproject with uv
uv add md2data
```

---

## Python Usage

```py
import md2data

# Basic conversion
markdown = "# Hello World\n\nThis is **bold** text."

# To JSON (default)
json_output = md2data.convert(markdown, "json")
print(json_output)

# To YAML
yaml_output = md2data.convert(markdown, "yaml")
print(yaml_output)

# To TOML
toml_output = md2data.convert(markdown, "toml")
print(toml_output)

# To XML
xml_output = md2data.convert(markdown, "xml")
print(xml_output)

# One-liner
print(md2data.convert("# Quick Test", "json"))
```

## Python - Reading from File

```py
import md2data

# Read Markdown file and convert
with open("README.md", "r", encoding="utf-8") as f:
    markdown = f.read()
    json_output = md2data.convert(markdown, "json")

# Write output
with open("output.json", "w", encoding="utf-8") as f:
    f.write(json_output)
```

## Python - Error Handling

```py
import md2data

try:
    result = md2data.convert("# Test", "json")
    print(result)
except ValueError as e:
    print(f"Invalid format: {e}")
except RuntimeError as e:
    print(f"Conversion error: {e}")

Python - Processing Multiple Files

import md2data
from pathlib import Path

markdown_files = Path("docs").glob("*.md")

for md_file in markdown_files:
    with open(md_file, "r") as f:
        markdown = f.read()

    json_output = md2data.convert(markdown, "json")

    output_file = md_file.with_suffix(".json")
    with open(output_file, "w") as f:
        f.write(json_output)

    print(f"Converted {md_file} → {output_file}")
```
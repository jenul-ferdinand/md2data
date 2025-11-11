# 🐍 Python

This document outlines all usage examples for Python development and production.

## Installation
```bash
# Install from local wheel
pip install bindings/python/dist/datadown-0.1.0-cp314-cp314-win_amd64.whl

# Install from PyPI
pip install datadown

# Install with uv
uv pip install datadown
# Add to pyproject with uv
uv add datadown
```

---

## Python Usage

```py
import datadown

# Basic conversion
markdown = "# Hello World\n\nThis is **bold** text."

# To JSON (default)
json_output = datadown.convert(markdown, "json")
print(json_output)

# To YAML
yaml_output = datadown.convert(markdown, "yaml")
print(yaml_output)

# To TOML
toml_output = datadown.convert(markdown, "toml")
print(toml_output)

# To XML
xml_output = datadown.convert(markdown, "xml")
print(xml_output)

# One-liner
print(datadown.convert("# Quick Test", "json"))
```

## Python - Reading from File

```py
import datadown

# Read Markdown file and convert
with open("README.md", "r", encoding="utf-8") as f:
    markdown = f.read()
    json_output = datadown.convert(markdown, "json")

# Write output
with open("output.json", "w", encoding="utf-8") as f:
    f.write(json_output)
```

## Python - Error Handling

```py
import datadown

try:
    result = datadown.convert("# Test", "json")
    print(result)
except ValueError as e:
    print(f"Invalid format: {e}")
except RuntimeError as e:
    print(f"Conversion error: {e}")

Python - Processing Multiple Files

import datadown
from pathlib import Path

markdown_files = Path("docs").glob("*.md")

for md_file in markdown_files:
    with open(md_file, "r") as f:
        markdown = f.read()

    json_output = datadown.convert(markdown, "json")

    output_file = md_file.with_suffix(".json")
    with open(output_file, "w") as f:
        f.write(json_output)

    print(f"Converted {md_file} → {output_file}")
```
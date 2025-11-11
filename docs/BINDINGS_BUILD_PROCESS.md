# Build process for Python and Node.js bindings

## Python

```bash
cd bindings/python
uv build # Builds using maturin
```

This will create `bindings/python/dist` with:

1. `datadown-(version)-cp314-win_amd64.whl` 
    - **What it is:** A Python wheel - the standard distributable format for Python packages.
    - **Contains:**
        - `datadown.py.cp314-win_amd64.pyd` - The compiled Rust code as a Python extension.
        - `__init__.py` - Python glue code to import the extension.
        - `METADATA` - Package information.
    - **Purpose:** Install with `pip install datadown-0.1.0-cp314-cp314-win_amd64.whl`

2. `datadown-(version).tar.gz`
    - **What it is:** A source distribution (sdist).
    - **Contains:** Source code that can be compiled on other platforms.
    - **Purpose:** For platforms where pre-built wheels aren't available.

---

## Node.js
```bash
cd bindings/node
npm run build # Builds using napi
```

This will create two files:

1. `index.node`
    - **What it is:** A native Node.js addon (compiled Rust as a DLL)
    - **Purpose:** Node.js loads this directly with require('./index.node')
2. `index.d.ts`
    - **What it is:** TypeScript type definitions
    - **Contains:** `export declare function convert(input: string, format: string): string`
    - **Purpose:** Enables autocomplete and type checking in TypeScript/JavaScript IDEs
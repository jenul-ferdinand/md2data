# Datadown for Node.js 📦🟢
> Markdown → JSON/YAML/TOML/XML. Faster with Rust.

## Installation
Install with npm.
```bash
npm install datadown
```

## Usage
```js
const { convert } = require('datadown');

const markdown = `# Hello World

This is a **markdown** document.
`;

const json = convert(markdown, 'json');
const yaml = convert(markdown, 'yaml');
const toml = convert(markdown, 'toml');
const xml = convert(markdown, 'xml');
```

## Developers: Build process
The build process can be initiated with `npm run build`.
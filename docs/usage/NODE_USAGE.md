# 📦 Node.js

## Installation

```bash
# Install from local package
cd bindings/node
npm install

# Link for local development
npm link

# Install from npm (after publishing)
npm install datadown
```

## JavaScript (CommonJS) Usage

```js
const { convert } = require('datadown');

// Basic conversion
const markdown = "# Hello World\n\nThis is **bold** text.";

// To JSON
const json = convert(markdown, "json");
console.log(json);

// To YAML
const yaml = convert(markdown, "yaml");
console.log(yaml);

// To TOML
const toml = convert(markdown, "toml");
console.log(toml);

// To XML
const xml = convert(markdown, "xml");
console.log(xml);

// One-liner
console.log(convert("# Quick Test", "json"));
```

## TypeScript (ESM) Usage

```ts
import { convert } from 'datadown';

const markdown: string = "# Hello World\n\nThis is **bold** text.";

// TypeScript knows the signature: (input: string, format: string) => string
const json: string = convert(markdown, "json");
console.log(json);

// With type safety
function convertMarkdown(md: string, format: "json" | "yaml" | "toml" | "xml"): string {
    return convert(md, format);
}

const result = convertMarkdown("# Test", "json");
```

## Node.js - Reading from File

```js
const fs = require('fs');
const { convert } = require('datadown');

// Read Markdown file
const markdown = fs.readFileSync('README.md', 'utf-8');

// Convert
const json = convert(markdown, 'json');

// Write output
fs.writeFileSync('output.json', json);
console.log('Conversion complete!');
```

## Node.js - Async/Promise Usage

```js
const fs = require('fs').promises;
const { convert } = require('datadown');

async function convertFile(inputPath, outputPath, format) {
    try {
        // Read input
        const markdown = await fs.readFile(inputPath, 'utf-8');

        // Convert (note: convert itself is synchronous)
        const output = convert(markdown, format);

        // Write output
        await fs.writeFile(outputPath, output);

        console.log(`Converted ${inputPath} → ${outputPath}`);
    } catch (error) {
        console.error('Error:', error.message);
    }
}

// Usage
convertFile('input.md', 'output.json', 'json');
```

## Node.js - Stream Processing

```js
const fs = require('fs');
const { convert } = require('datadown');

// Read entire file first (since convert needs full input)
let chunks = [];

fs.createReadStream('large-file.md', 'utf-8')
    .on('data', (chunk) => chunks.push(chunk))
    .on('end', () => {
        const markdown = chunks.join('');
        const json = convert(markdown, 'json');

        fs.writeFileSync('output.json', json);
        console.log('Done!');
    });
```

## Node.js - Express API Example

```js
const express = require('express');
const { convert } = require('datadown');

const app = express();
app.use(express.text({ type: 'text/markdown' }));

app.post('/convert', (req, res) => {
    const markdown = req.body;
    const format = req.query.format || 'json';

    try {
        const output = convert(markdown, format);

        const contentTypes = {
            json: 'application/json',
            yaml: 'application/x-yaml',
            toml: 'application/toml',
            xml: 'application/xml'
        };

        res.type(contentTypes[format] || 'text/plain');
        res.send(output);
    } catch (error) {
        res.status(400).json({ error: error.message });
    }
});

app.listen(3000, () => {
    console.log('Datadown API running on http://localhost:3000');
});

// Usage:
// curl -X POST http://localhost:3000/convert?format=json \
//   -H "Content-Type: text/markdown" \
//   -d "# Hello World"
```
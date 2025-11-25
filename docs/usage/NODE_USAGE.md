# 📦 Node.js

## Installation methods
```bash
# 1. Install from local package
cd bindings/node
npm install
npm link

# 2. Install from npm
npm install md2data
```

## E.g., CommonJS basic usage
```js
const { convert } = require('md2data');

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

## E.g., TypeScript (ESM) usage
```ts
import { convert } from 'md2data';

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

## E.g., Reading from a file
```js
const fs = require('fs');
const { convert } = require('md2data');

// Read Markdown file
const markdown = fs.readFileSync('README.md', 'utf-8');

// Convert
const json = convert(markdown, 'json');

// Write output
fs.writeFileSync('output.json', json);
console.log('Conversion complete!');
```

## E.g., Async/Promise usage
```js
const fs = require('fs').promises;
const { convert } = require('md2data');

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

## E.g., Stream processing
```js
const fs = require('fs');
const { convert } = require('md2data');

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

## E.g., Express API example
```js
const express = require('express');
const { convert } = require('md2data');

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
    console.log('MD2Data API running on http://localhost:3000');
});

// Usage:
// curl -X POST http://localhost:3000/convert?format=json \
//   -H "Content-Type: text/markdown" \
//   -d "# Hello World"
```
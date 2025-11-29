const { convert } = require('../index.js');

describe("JSON Formatting", () => {
    test("should convert markdown to JSON", () => {
        const md = "# Hello World\n\nThis is **bold**.";
        const res = convert(md, 'json');

        const parsed = JSON.parse(res);
        expect(parsed).toBeDefined();
        // TODO: Add assertions about the structure
    })
})
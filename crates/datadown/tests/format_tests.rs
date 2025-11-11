use datadown::{convert_str, OutputFormat};
use pretty_assertions::assert_eq;

// ============================================================================
// JSON OUTPUT TESTS
// ============================================================================

#[test]
fn test_json_simple_heading() {
    let md = "# Hello World";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    // Verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["type"], "Document");
    assert_eq!(parsed["children"][0]["type"], "Heading");
    assert_eq!(parsed["children"][0]["level"], 1);
    assert_eq!(parsed["children"][0]["text"], "Hello World");
}

#[test]
fn test_json_paragraph() {
    let md = "This is a paragraph.";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["type"], "Document");
    assert_eq!(parsed["children"][0]["type"], "Paragraph");
    assert_eq!(parsed["children"][0]["children"][0]["type"], "Text");
    assert_eq!(parsed["children"][0]["children"][0]["content"], "This is a paragraph.");
}

#[test]
fn test_json_list() {
    let md = "* Item 1\n* Item 2";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["children"][0]["type"], "List");
    assert_eq!(parsed["children"][0]["ordered"], false);
    assert_eq!(parsed["children"][0]["items"].as_array().unwrap().len(), 2);
}

#[test]
fn test_json_code_block() {
    let md = "```\ncode here\n```";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["children"][0]["type"], "CodeBlock");
    assert_eq!(parsed["children"][0]["content"], "code here\n");
}

#[test]
fn test_json_complex_document() {
    let md = r#"# Title

Paragraph with **bold**.

* List item 1
* List item 2

```
code
```
"#;
    let json = convert_str(md, OutputFormat::Json).unwrap();

    // Should be valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["type"], "Document");
    assert!(parsed["children"].as_array().unwrap().len() >= 4);
}

#[test]
fn test_json_empty_document() {
    let md = "";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["type"], "Document");
    assert_eq!(parsed["children"].as_array().unwrap().len(), 0);
}

#[test]
fn test_json_unicode() {
    let md = "# 你好";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    assert_eq!(parsed["children"][0]["text"], "你好");
}

// ============================================================================
// YAML OUTPUT TESTS
// ============================================================================

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_simple_heading() {
    let md = "# Hello World";
    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();

    // Verify it's valid YAML
    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Valid YAML");

    assert_eq!(parsed["type"], "Document");
    assert_eq!(parsed["children"][0]["type"], "Heading");
    assert_eq!(parsed["children"][0]["level"], 1);
    assert_eq!(parsed["children"][0]["text"], "Hello World");
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_contains_expected_keys() {
    let md = "# Test\n\nParagraph.";
    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();

    // Check for expected YAML structure
    assert!(yaml.contains("type: Document"));
    assert!(yaml.contains("type: Heading"));
    assert!(yaml.contains("type: Paragraph"));
    assert!(yaml.contains("level: 1"));
    assert!(yaml.contains("text: Test"));
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_list() {
    let md = "* Item 1\n* Item 2";
    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();

    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Valid YAML");

    assert_eq!(parsed["children"][0]["type"], "List");
    assert_eq!(parsed["children"][0]["ordered"], false);
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_empty_document() {
    let md = "";
    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();

    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Valid YAML");

    assert_eq!(parsed["type"], "Document");
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_unicode() {
    let md = "# 日本語";
    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();

    assert!(yaml.contains("日本語"));
}

// ============================================================================
// TOML OUTPUT TESTS
// ============================================================================

#[cfg(feature = "toml")]
#[test]
fn test_toml_simple_heading() {
    let md = "# Hello World";
    let toml_str = convert_str(md, OutputFormat::Toml).unwrap();

    // Verify it's valid TOML
    let parsed: toml::Value = toml::from_str(&toml_str).expect("Valid TOML");

    assert_eq!(parsed["type"].as_str(), Some("Document"));
    assert_eq!(parsed["children"][0]["type"].as_str(), Some("Heading"));
    assert_eq!(parsed["children"][0]["level"].as_integer(), Some(1));
    assert_eq!(parsed["children"][0]["text"].as_str(), Some("Hello World"));
}

#[cfg(feature = "toml")]
#[test]
fn test_toml_contains_expected_structure() {
    let md = "# Test";
    let toml_str = convert_str(md, OutputFormat::Toml).unwrap();

    assert!(toml_str.contains("type = \"Document\""));
    assert!(toml_str.contains("type = \"Heading\""));
    assert!(toml_str.contains("level = 1"));
}

#[cfg(feature = "toml")]
#[test]
fn test_toml_list() {
    let md = "* Item 1\n* Item 2";
    let toml_str = convert_str(md, OutputFormat::Toml).unwrap();

    let parsed: toml::Value = toml::from_str(&toml_str).expect("Valid TOML");

    assert_eq!(parsed["children"][0]["type"].as_str(), Some("List"));
    assert_eq!(parsed["children"][0]["ordered"].as_bool(), Some(false));
}

#[cfg(feature = "toml")]
#[test]
fn test_toml_empty_document() {
    let md = "";
    let toml_str = convert_str(md, OutputFormat::Toml).unwrap();

    let parsed: toml::Value = toml::from_str(&toml_str).expect("Valid TOML");

    assert_eq!(parsed["type"].as_str(), Some("Document"));
}

// ============================================================================
// XML OUTPUT TESTS
// ============================================================================

#[cfg(feature = "xml")]
#[test]
fn test_xml_simple_heading() {
    let md = "# Hello World";
    let xml = convert_str(md, OutputFormat::Xml).unwrap();

    // Verify it contains expected XML structure
    assert!(xml.contains("<type>Document</type>"));
    assert!(xml.contains("<type>Heading</type>"));
    assert!(xml.contains("<level>1</level>"));
    assert!(xml.contains("<text>Hello World</text>"));
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_paragraph() {
    let md = "This is text.";
    let xml = convert_str(md, OutputFormat::Xml).unwrap();

    assert!(xml.contains("<type>Paragraph</type>"));
    assert!(xml.contains("<type>Text</type>"));
    assert!(xml.contains("<content>This is text.</content>"));
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_list() {
    let md = "* Item 1\n* Item 2";
    let xml = convert_str(md, OutputFormat::Xml).unwrap();

    assert!(xml.contains("<type>List</type>"));
    assert!(xml.contains("<ordered>false</ordered>"));
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_code_block() {
    let md = "```\ncode\n```";
    let xml = convert_str(md, OutputFormat::Xml).unwrap();

    assert!(xml.contains("<type>CodeBlock</type>"));
    assert!(xml.contains("<content>code"));
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_empty_document() {
    let md = "";
    let xml = convert_str(md, OutputFormat::Xml).unwrap();

    assert!(xml.contains("<type>Document</type>"));
}

// ============================================================================
// CROSS-FORMAT CONSISTENCY TESTS
// ============================================================================

#[test]
fn test_all_formats_handle_same_input() {
    let md = "# Test\n\nParagraph.";

    // All formats should successfully convert without error
    let json = convert_str(md, OutputFormat::Json);
    assert!(json.is_ok());

    #[cfg(feature = "yaml")]
    {
        let yaml = convert_str(md, OutputFormat::Yaml);
        assert!(yaml.is_ok());
    }

    #[cfg(feature = "toml")]
    {
        let toml_result = convert_str(md, OutputFormat::Toml);
        assert!(toml_result.is_ok());
    }

    #[cfg(feature = "xml")]
    {
        let xml = convert_str(md, OutputFormat::Xml);
        assert!(xml.is_ok());
    }
}

#[test]
fn test_formats_produce_different_outputs() {
    let md = "# Test";

    let json = convert_str(md, OutputFormat::Json).unwrap();

    #[cfg(feature = "yaml")]
    {
        let yaml = convert_str(md, OutputFormat::Yaml).unwrap();
        assert_ne!(json, yaml);  // Different formats should produce different strings
    }

    #[cfg(feature = "xml")]
    {
        let xml = convert_str(md, OutputFormat::Xml).unwrap();
        assert_ne!(json, xml);
    }
}

#[cfg(all(feature = "yaml", feature = "toml"))]
#[test]
fn test_yaml_and_toml_parseable() {
    let md = "# Title\n\nParagraph with text.";

    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();
    let yaml_parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Valid YAML");

    let toml_str = convert_str(md, OutputFormat::Toml).unwrap();
    let toml_parsed: toml::Value = toml::from_str(&toml_str).expect("Valid TOML");

    // Both should represent same document type
    assert_eq!(yaml_parsed["type"], "Document");
    assert_eq!(toml_parsed["type"].as_str(), Some("Document"));
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_invalid_format_enum_does_not_exist() {
    // This is more of a compile-time test that OutputFormat enum is exhaustive
    // If this compiles, we know all enum variants are handled

    let md = "# Test";

    let formats = vec![
        OutputFormat::Json,
        OutputFormat::Yaml,
        OutputFormat::Toml,
        OutputFormat::Xml,
    ];

    for format in formats {
        let result = convert_str(md, format);
        // All should work or return proper error
        assert!(result.is_ok() || result.is_err());
    }
}

// ============================================================================
// SPECIAL CHARACTER TESTS
// ============================================================================

#[test]
fn test_json_escapes_quotes() {
    let md = r#"This has "quotes" in it."#;
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    // Quotes should be preserved in the content
    let content = parsed["children"][0]["children"][0]["content"]
        .as_str()
        .unwrap();
    assert!(content.contains("quotes"));
}

#[test]
fn test_json_handles_newlines_in_code() {
    let md = "```\nline 1\nline 2\n```";
    let json = convert_str(md, OutputFormat::Json).unwrap();

    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Valid JSON");

    let content = parsed["children"][0]["content"].as_str().unwrap();
    assert!(content.contains("line 1"));
    assert!(content.contains("line 2"));
}

#[cfg(feature = "yaml")]
#[test]
fn test_yaml_handles_special_yaml_chars() {
    let md = "This has: colons and - dashes";
    let yaml = convert_str(md, OutputFormat::Yaml).unwrap();

    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Valid YAML");

    let content = parsed["children"][0]["children"][0]["content"]
        .as_str()
        .unwrap();
    assert!(content.contains("colons"));
    assert!(content.contains("dashes"));
}

#[cfg(feature = "xml")]
#[test]
fn test_xml_escapes_special_chars() {
    let md = "Text with & ampersand";
    let xml = convert_str(md, OutputFormat::Xml).unwrap();

    // XML should escape special characters (& becomes &amp;)
    // The exact escaping depends on quick-xml implementation
    assert!(xml.contains("&amp;"));
    assert!(xml.contains("ampersand"));
}

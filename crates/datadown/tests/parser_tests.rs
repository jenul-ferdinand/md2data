use datadown::parse_markdown;
use datadown::Node;
use pretty_assertions::assert_eq;

// ============================================================================
// HEADING TESTS
// ============================================================================

#[test]
fn test_simple_heading_level_1() {
    let md = "# Hello World";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Heading { level, text } => {
                    assert_eq!(*level, 1);
                    assert_eq!(text, "Hello World");
                }
                other => panic!("Expected Heading, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_all_heading_levels() {
    let md = "# H1\n## H2\n### H3\n#### H4\n##### H5\n###### H6";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 6);

            for (i, expected_level) in (1..=6).enumerate() {
                match &children[i] {
                    Node::Heading { level, text } => {
                        assert_eq!(*level, expected_level);
                        assert_eq!(text, &format!("H{}", expected_level));
                    }
                    other => panic!("Expected Heading at index {}, got {:?}", i, other),
                }
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_heading_with_inline_formatting() {
    let md = "# Hello **bold** and *italic* world";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Heading { level, text } => {
                    assert_eq!(*level, 1);
                    // Formatting should be stripped but text preserved
                    assert_eq!(text, "Hello bold and italic world");
                }
                other => panic!("Expected Heading, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_empty_heading() {
    let md = "#";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Heading { level, text } => {
                    assert_eq!(*level, 1);
                    assert_eq!(text, "");
                }
                other => panic!("Expected Heading, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

// ============================================================================
// PARAGRAPH TESTS
// ============================================================================

#[test]
fn test_simple_paragraph() {
    let md = "This is a simple paragraph.";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    assert_eq!(para_children.len(), 1);
                    match &para_children[0] {
                        Node::Text { content } => {
                            assert_eq!(content, "This is a simple paragraph.");
                        }
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_multiple_paragraphs() {
    let md = "First paragraph.\n\nSecond paragraph.";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 2);

            // Check first paragraph
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    assert_eq!(para_children.len(), 1);
                    match &para_children[0] {
                        Node::Text { content } => assert_eq!(content, "First paragraph."),
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }

            // Check second paragraph
            match &children[1] {
                Node::Paragraph { children: para_children } => {
                    assert_eq!(para_children.len(), 1);
                    match &para_children[0] {
                        Node::Text { content } => assert_eq!(content, "Second paragraph."),
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_paragraph_with_bold_text() {
    let md = "This has **bold text** in it.";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    // Should have 3 text nodes: before, bold content, after
                    assert_eq!(para_children.len(), 3);

                    match &para_children[0] {
                        Node::Text { content } => assert_eq!(content, "This has "),
                        other => panic!("Expected Text, got {:?}", other),
                    }

                    match &para_children[1] {
                        Node::Text { content } => assert_eq!(content, "bold text"),
                        other => panic!("Expected Text, got {:?}", other),
                    }

                    match &para_children[2] {
                        Node::Text { content } => assert_eq!(content, " in it."),
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_paragraph_with_italic_text() {
    let md = "This has *italic text* in it.";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    assert_eq!(para_children.len(), 3);

                    match &para_children[1] {
                        Node::Text { content } => assert_eq!(content, "italic text"),
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_paragraph_with_inline_code() {
    let md = "This has `inline code` in it.";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    // Inline code is merged into the text content, not separated
                    assert_eq!(para_children.len(), 1);

                    match &para_children[0] {
                        Node::Text { content } => {
                            // Text should contain the inline code content (backticks removed)
                            assert_eq!(content, "This has inline code in it.");
                        }
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

// ============================================================================
// LIST TESTS
// ============================================================================

#[test]
fn test_simple_unordered_list() {
    let md = "* Item 1\n* Item 2\n* Item 3";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::List { ordered, items } => {
                    assert_eq!(*ordered, false);
                    assert_eq!(items.len(), 3);

                    for (i, item) in items.iter().enumerate() {
                        match item {
                            Node::ListItem { children: item_children } => {
                                assert_eq!(item_children.len(), 1);
                                match &item_children[0] {
                                    Node::Text { content } => {
                                        assert_eq!(content, &format!("Item {}", i + 1));
                                    }
                                    other => panic!("Expected Text, got {:?}", other),
                                }
                            }
                            other => panic!("Expected ListItem, got {:?}", other),
                        }
                    }
                }
                other => panic!("Expected List, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_simple_ordered_list() {
    let md = "1. First\n2. Second\n3. Third";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::List { ordered, items } => {
                    assert_eq!(*ordered, true);
                    assert_eq!(items.len(), 3);
                }
                other => panic!("Expected List, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_nested_lists() {
    let md = "* Item 1\n* Item 2\n  * Nested 1\n  * Nested 2\n* Item 3";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::List { ordered, items } => {
                    assert_eq!(*ordered, false);
                    assert_eq!(items.len(), 3);

                    // Check second item has nested list
                    match &items[1] {
                        Node::ListItem { children: item_children } => {
                            assert_eq!(item_children.len(), 2);  // Text + nested list

                            // Check nested list
                            match &item_children[1] {
                                Node::List { ordered: nested_ordered, items: nested_items } => {
                                    assert_eq!(*nested_ordered, false);
                                    assert_eq!(nested_items.len(), 2);
                                }
                                other => panic!("Expected nested List, got {:?}", other),
                            }
                        }
                        other => panic!("Expected ListItem, got {:?}", other),
                    }
                }
                other => panic!("Expected List, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_list_with_bold_items() {
    let md = "* **Bold item**\n* Normal item";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::List { items, .. } => {
                    assert_eq!(items.len(), 2);

                    // First item should have text "Bold item" (formatting stripped)
                    match &items[0] {
                        Node::ListItem { children: item_children } => {
                            assert_eq!(item_children.len(), 1);
                            match &item_children[0] {
                                Node::Text { content } => assert_eq!(content, "Bold item"),
                                other => panic!("Expected Text, got {:?}", other),
                            }
                        }
                        other => panic!("Expected ListItem, got {:?}", other),
                    }
                }
                other => panic!("Expected List, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

// ============================================================================
// CODE BLOCK TESTS
// ============================================================================

#[test]
fn test_simple_code_block() {
    let md = "```\nlet x = 42;\n```";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::CodeBlock { info, content } => {
                    assert_eq!(*info, None);
                    assert_eq!(content, "let x = 42;\n");
                }
                other => panic!("Expected CodeBlock, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_code_block_with_language() {
    let md = "```rust\nfn main() {}\n```";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::CodeBlock { info, content } => {
                    // Currently info is always None (known limitation)
                    // This test documents current behavior
                    assert_eq!(*info, None);
                    assert_eq!(content, "fn main() {}\n");
                }
                other => panic!("Expected CodeBlock, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_empty_code_block() {
    let md = "```\n```";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::CodeBlock { content, .. } => {
                    assert_eq!(content, "");
                }
                other => panic!("Expected CodeBlock, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_indented_code_block() {
    let md = "    indented code\n    more code";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::CodeBlock { content, .. } => {
                    assert!(content.contains("indented code"));
                }
                other => panic!("Expected CodeBlock, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

// ============================================================================
// MIXED CONTENT TESTS
// ============================================================================

#[test]
fn test_heading_and_paragraph() {
    let md = "# Title\n\nThis is a paragraph.";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 2);

            match &children[0] {
                Node::Heading { level, text } => {
                    assert_eq!(*level, 1);
                    assert_eq!(text, "Title");
                }
                other => panic!("Expected Heading, got {:?}", other),
            }

            match &children[1] {
                Node::Paragraph { .. } => {},
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_complex_document() {
    let md = r#"# Main Title

This is a paragraph with **bold** and *italic* text.

## Subheading

* List item 1
* List item 2
  * Nested item

```
code block
```

Final paragraph.
"#;
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            // Should have: heading, paragraph, heading, list, code block, paragraph
            assert_eq!(children.len(), 6);

            // Verify types in order
            assert!(matches!(&children[0], Node::Heading { level: 1, .. }));
            assert!(matches!(&children[1], Node::Paragraph { .. }));
            assert!(matches!(&children[2], Node::Heading { level: 2, .. }));
            assert!(matches!(&children[3], Node::List { .. }));
            assert!(matches!(&children[4], Node::CodeBlock { .. }));
            assert!(matches!(&children[5], Node::Paragraph { .. }));
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

#[test]
fn test_empty_document() {
    let md = "";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 0);
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_whitespace_only() {
    let md = "   \n\n   ";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            // Should have no content or just empty nodes
            // This documents current behavior
            assert!(children.is_empty() || children.len() <= 1);
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_parser_does_not_panic_on_inline_formatting() {
    // This is a regression test for the panic bug we fixed
    let md = "# Authors\n\n* Nate Vack\n* **Vendor Packages**\n  * docopt\n  * CommonMark-py";
    let ast = parse_markdown(md);

    // Should not panic and should parse successfully
    match ast {
        Node::Document { children } => {
            assert!(children.len() >= 2);
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_soft_break_becomes_space() {
    let md = "Line one\nLine two";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    assert_eq!(para_children.len(), 1);
                    match &para_children[0] {
                        Node::Text { content } => {
                            assert_eq!(content, "Line one Line two");
                        }
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_hard_break_becomes_space() {
    let md = "Line one  \nLine two";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 1);
            match &children[0] {
                Node::Paragraph { children: para_children } => {
                    // Hard breaks also become spaces in our implementation
                    assert_eq!(para_children.len(), 1);
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

#[test]
fn test_unicode_content() {
    let md = "# 你好世界\n\nこんにちは 🌍";
    let ast = parse_markdown(md);

    match ast {
        Node::Document { children } => {
            assert_eq!(children.len(), 2);

            match &children[0] {
                Node::Heading { text, .. } => {
                    assert_eq!(text, "你好世界");
                }
                other => panic!("Expected Heading, got {:?}", other),
            }

            match &children[1] {
                Node::Paragraph { children: para_children } => {
                    match &para_children[0] {
                        Node::Text { content } => {
                            assert_eq!(content, "こんにちは 🌍");
                        }
                        other => panic!("Expected Text, got {:?}", other),
                    }
                }
                other => panic!("Expected Paragraph, got {:?}", other),
            }
        }
        other => panic!("Expected Document, got {:?}", other),
    }
}

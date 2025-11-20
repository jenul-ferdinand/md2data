use pulldown_cmark::{Event, Parser, Tag, TagEnd };
use crate::ast::Node;
use crate::ast::NodeOrString;

/// Takes markdown text and outputs an AST
/// 
/// Uses pulldown_cmark to build AST, see ast.rs for structure.
pub fn parse_markdown(input: &str) -> Node {
    let parser = Parser::new(input);
    let mut stack: Vec<Node> = vec![Node::Document { children: vec![] }];
    let mut text_buf = String::new();

    for ev in parser {
        match ev {
            Event::Start(tag) => {
                flush_text(&mut stack, &mut text_buf);

                match tag {
                    Tag::Heading { level, .. } => {
                        stack.push(Node::Heading {
                            level: level as u8,
                            children: vec![]
                        })
                    }
                    Tag::Paragraph => {
                        stack.push(Node::Paragraph { 
                            children: vec![] 
                        }) 
                    },
                    Tag::List(start) => {
                        let ordered = start.is_some(); // Some(n) => ordered list starting at n; None => unordered
                        stack.push(Node::List { 
                            ordered, items: vec![] 
                        })
                    }
                    Tag::Item => { 
                        stack.push(Node::ListItem { 
                            children: vec![] 
                        }) 
                    },
                    Tag::CodeBlock(_kind) => {
                        // pulldown-cmark 0.10 changed fenced info representation; keep it simple for now.
                        stack.push(Node::CodeBlock { 
                            info: None, 
                            content: String::new() 
                        })
                    }
                    _ => {}
                }
            }

            Event::End(tag_end) => {
                flush_text(&mut stack, &mut text_buf);

                match tag_end {
                    TagEnd::Heading { .. } |
                    TagEnd::Paragraph |
                    TagEnd::Item |
                    TagEnd::List(_) |
                    TagEnd::CodeBlock => {
                        let node = stack.pop().expect("node on stack");
                        push_node_to_parent(&mut stack, node);
                    }

                    _ => {}
                }
            }

            Event::Text(t) | Event::Code(t) => {
                text_buf.push_str(&t);
            }

            Event::SoftBreak | Event::HardBreak => {
                text_buf.push(' ');
            }

            _ => {}
        }
    }

    flush_text(&mut stack, &mut text_buf);
    stack.pop().expect("document at root")
}

/// Moves accumulated text from the buffer to the current node
fn flush_text(stack: &mut Vec<Node>, buf: &mut String) {
    if buf.is_empty() { return; }

    let text = buf.clone();
    buf.clear();

    if let Some(top) = stack.last_mut() {
        match top {
            // For containers, we push NodeOrString::String
            Node::Document { children } => children.push(NodeOrString::String(text)),
            Node::Paragraph { children } => children.push(NodeOrString::String(text)),
            Node::Heading { children, .. } => children.push(NodeOrString::String(text)),
            Node::ListItem { children } => children.push(NodeOrString::String(text)),

            // CodeBlock is special: it handles raw content string directly
            Node::CodeBlock { content, .. } => content.push_str(&text),

            _ => {}
        }
    }
}

/// Adds a completed child node to its parent container
fn push_node_to_parent(stack: &mut Vec<Node>, node: Node) {
    if let Some(parent) = stack.last_mut() {
        match parent {
            // Standard containers take NodeOrString
            Node::Document { children } => children.push(NodeOrString::Node(Box::new(node))),
            Node::Paragraph { children } => children.push(NodeOrString::Node(Box::new(node))),
            Node::Heading { children, .. } => children.push(NodeOrString::Node(Box::new(node))),
            Node::ListItem { children } => children.push(NodeOrString::Node(Box::new(node))),

            // List is special: it only accepts Node (specifically ListItem), not strings
            Node::List { items, .. } => items.push(node),

            _ => {}
        }
    }
}

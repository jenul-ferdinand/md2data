use pulldown_cmark::{Event, Parser, Tag, TagEnd };
use crate::ast::Node;

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
                        stack.push(Node::Heading { level: level as u8, text: String::new() })
                    }
                    Tag::Paragraph => stack.push(Node::Paragraph { children: vec![] }),
                    Tag::List(start) => {
                        let ordered = start.is_some(); // Some(n) => ordered list starting at n; None => unordered
                        stack.push(Node::List { ordered, items: vec![] })
                    }
                    Tag::Item => stack.push(Node::ListItem { children: vec![] }),
                    Tag::CodeBlock(_kind) => {
                        // pulldown-cmark 0.10 changed fenced info representation; keep it simple for now.
                        stack.push(Node::CodeBlock { info: None, content: String::new() })
                    }
                    _ => {}
                }
            }

            Event::End(tag_end) => {
                flush_text(&mut stack, &mut text_buf);

                // Only pop for tags we actually pushed to the stack
                match tag_end {
                    TagEnd::Heading { .. } => {
                        let node = stack.pop().expect("heading on stack");
                        append_to_heading_text(&mut stack, node);
                    }
                    TagEnd::Paragraph | TagEnd::Item | TagEnd::List(_) | TagEnd::CodeBlock => {
                        let node = stack.pop().expect("node on stack");
                        push_child(&mut stack, node);
                    }
                    _ => {
                        // For inline elements (Strong, Emphasis, Link, etc.) that we don't track:
                        // Don't pop anything - their text content was already flushed to the parent
                    }
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

fn flush_text(stack: &mut Vec<Node>, buf: &mut String) {
    if buf.is_empty() { return; }
    if let Some(top) = stack.last_mut() {
        match top {
            Node::Heading { text, .. } => text.push_str(buf),
            Node::Paragraph { children } => children.push(Node::Text { content: buf.clone() }),
            Node::CodeBlock { content, .. } => content.push_str(buf),
            Node::ListItem { children } => children.push(Node::Text { content: buf.clone() }),
            _ => {}
        }
    }
    buf.clear();
}

fn push_child(stack: &mut Vec<Node>, node: Node) {
    if let Some(parent) = stack.last_mut() {
        match parent {
            Node::Document { children } => children.push(node),
            Node::Paragraph { children } => children.push(node),
            Node::List { items, .. } => items.push(node),
            Node::ListItem { children } => children.push(node),
            _ => {}
        }
    }
}

fn append_to_heading_text(stack: &mut Vec<Node>, node: Node) {
    push_child(stack, node);
}

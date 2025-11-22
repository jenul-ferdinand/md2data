use pulldown_cmark::{Event, Parser, Tag, TagEnd, CodeBlockKind};
use crate::ast::MinifiedNode;

/// Takes markdown text and converts to an MinifiedNode AST
/// 
/// This is a completely different type of parsing compared to parse.rs
pub fn parse_markdown_minified(input: &str) -> MinifiedNode {
    let parser = Parser::new(input);
    
    // Root is always a Map
    let mut root = MinifiedNode::Map(vec![]);
    let mut heading_stack: Vec<(u8, String)> = vec![];
    
    // Buffer for capturing text content
    let mut text_buf = String::new();
    let mut pending_heading_level: Option<u8> = None;
    
    // Temporary storage for content under the current key
    let mut current_content: Vec<MinifiedNode> = vec![];

    let events: Vec<Event> = parser.collect();
    let mut iter = events.into_iter().peekable();

    while let Some(ev) = iter.next() {
        match ev {
            // Headings become keys
            Event::Start(Tag::Heading { level, .. }) => {
                // 1. Flush previous content to the current tip before switching keys
                if !heading_stack.is_empty() && !current_content.is_empty() {
                    append_content_to_tip(&mut root, &heading_stack, std::mem::take(&mut current_content));
                }
                
                pending_heading_level = Some(level as u8);
                text_buf.clear();
            }
            Event::End(TagEnd::Heading { .. }) => {
                if let Some(level) = pending_heading_level {
                    let key = text_buf.trim().to_string();
                    
                    // Pop stack if we are going back up a level
                    while let Some((last_lvl, _)) = heading_stack.last() {
                        if *last_lvl >= level {
                            heading_stack.pop();
                        } else {
                            break;
                        }
                    }
                    
                    // Create the path
                    ensure_path_is_map(&mut root, &heading_stack);
                    
                    // Add the new key (initially empty)
                    add_key_to_map(&mut root, &heading_stack, key.clone());
                    
                    heading_stack.push((level, key));
                }
                pending_heading_level = None;
                text_buf.clear();
            }

            // Code blocks, keeping fences (e.g., ```)
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang = match kind {
                    CodeBlockKind::Fenced(l) => l.to_string(),
                    CodeBlockKind::Indented => "".to_string(),
                };
                text_buf.push_str("```");
                text_buf.push_str(&lang);
                text_buf.push('\n');
            }
            Event::End(TagEnd::CodeBlock) => {
                text_buf.push_str("```\n");
                current_content.push(MinifiedNode::String(text_buf.clone()));
                text_buf.clear();
            }

            // Text
            Event::Text(t) | Event::Code(t) => {
                text_buf.push_str(&t);
            }
            Event::SoftBreak | Event::HardBreak => {
                text_buf.push('\n');
            }
            Event::End(TagEnd::Paragraph) => {
                let val = text_buf.trim().to_string();
                if !val.is_empty() {
                    current_content.push(MinifiedNode::String(val));
                }
                text_buf.clear();
            }

            // Lists
            Event::Start(Tag::List(_)) => {
                let list_node = parse_recursive_list(&mut iter);
                current_content.push(list_node);
            }

            _ => {}
        }
    }

    // Final Flush
    if !heading_stack.is_empty() && !current_content.is_empty() {
        append_content_to_tip(&mut root, &heading_stack, current_content);
    }

    root
}

fn parse_recursive_list(iter: &mut std::iter::Peekable<std::vec::IntoIter<Event>>) -> MinifiedNode {
    let mut items = vec![];
    let mut text_buf = String::new();

    while let Some(ev) = iter.next() {
        match ev {
            Event::End(TagEnd::List(_)) => break,
            
            Event::Start(Tag::Item) => {
                text_buf.clear();
                
                let mut item_parts: Vec<MinifiedNode> = vec![];

                while let Some(sub_ev) = iter.peek() {
                    match sub_ev {
                        Event::End(TagEnd::Item) => { 
                            iter.next(); 
                            break; 
                        },
                        Event::Start(Tag::List(_)) => {
                            let txt = text_buf.trim().to_string();
                            if !txt.is_empty() {
                                item_parts.push(MinifiedNode::String(txt));
                                text_buf.clear();
                            }
                            
                            iter.next(); 
                            let nested = parse_recursive_list(iter);
                            item_parts.push(nested);
                        }
                        Event::Text(t) | Event::Code(t) => {
                            text_buf.push_str(t);
                            iter.next();
                        }
                        Event::SoftBreak | Event::HardBreak => {
                            text_buf.push(' ');
                            iter.next();
                        }
                        _ => { iter.next(); }
                    }
                }

                let txt = text_buf.trim().to_string();
                if !txt.is_empty() {
                    item_parts.push(MinifiedNode::String(txt));
                }

                if item_parts.len() == 1 {
                    items.push(item_parts[0].clone());
                } else {
                    for part in item_parts {
                        items.push(part);
                    }
                }
            }
            _ => {}
        }
    }
    MinifiedNode::Array(items)
}

/// --- Helper logic ---

fn ensure_path_is_map(root: &mut MinifiedNode, path: &[(u8, String)]) {
    let mut current = root;
    
    for (_, segment) in path {
        // 1. Force Current to be Map
        if !matches!(current, MinifiedNode::Map(_)) {
            *current = MinifiedNode::Map(vec![]);
        }
        
        // 2. Traverse down
        // We use a block here to limit the borrow scope of 'map'
        match current {
            MinifiedNode::Map(map) => {
                if let Some(idx) = map.iter().position(|(k, _)| k == segment) {
                    current = &mut map[idx].1;
                } else {
                    map.push((segment.clone(), MinifiedNode::Map(vec![])));
                    let last_idx = map.len() - 1;
                    current = &mut map[last_idx].1;
                }
            }
            _ => unreachable!(),
        }
    }

    // 3. Clobber tip if it exists as a String
    if !matches!(current, MinifiedNode::Map(_)) {
        *current = MinifiedNode::Map(vec![]);
    }
}

fn add_key_to_map(root: &mut MinifiedNode, path: &[(u8, String)], key: String) {
    let mut current = root;
    // Traverse to parent
    for (_, segment) in path {
        match current {
            MinifiedNode::Map(map) => {
                // We expect the path to exist because ensure_path_is_map was called
                let idx = map.iter().position(|(k, _)| k == segment)
                    .expect("Path broken in add_key_to_map");
                current = &mut map[idx].1;
            }
            _ => panic!("Parent path is not a Map"),
        }
    }
    // Insert key at parent
    if let MinifiedNode::Map(map) = current {
        map.push((key, MinifiedNode::String(String::new())));
    }
}

fn append_content_to_tip(root: &mut MinifiedNode, path: &[(u8, String)], content: Vec<MinifiedNode>) {
    if path.is_empty() { return; }
    
    let (_key_lvl, key_str) = path.last().unwrap();
    let parent_path = &path[0..path.len()-1];
    
    let mut current = root;
    for (_, segment) in parent_path {
        match current {
            MinifiedNode::Map(map) => {
                if let Some(idx) = map.iter().position(|(k, _)| k == segment) {
                    current = &mut map[idx].1;
                } else { return; } 
            }
            _ => return,
        }
    }

    if let MinifiedNode::Map(map) = current {
        if let Some(idx) = map.iter().position(|(k, _)| k == key_str) {
            let target = &mut map[idx].1;
            
            // Conflict Rule: Subheadings win. Content ignored/clobbered.
            if matches!(target, MinifiedNode::Map(_)) {
                return;
            }

            // Heuristic: Pure List vs Mixed Content
            let has_text = content.iter().any(|n| matches!(n, MinifiedNode::String(_)));
            let has_list = content.iter().any(|n| matches!(n, MinifiedNode::Array(_)));

            if !has_text && has_list && content.len() == 1 {
                *target = content[0].clone();
            } else {
                let mut combined_string = String::new();
                
                if let MinifiedNode::String(s) = target {
                    if !s.is_empty() {
                        combined_string.push_str(s);
                        combined_string.push_str("\n\n");
                    }
                }

                for node in content {
                    match node {
                        MinifiedNode::String(s) => {
                            combined_string.push_str(&s);
                            combined_string.push_str("\n\n");
                        }
                        MinifiedNode::Array(arr) => {
                            for item in arr {
                                stringify_list_item(&mut combined_string, &item, 0);
                            }
                            combined_string.push('\n');
                        }
                        _ => {}
                    }
                }
                
                *target = MinifiedNode::String(combined_string.trim().to_string());
            }
        }
    }
}

fn stringify_list_item(buf: &mut String, node: &MinifiedNode, indent: usize) {
    let spaces = " ".repeat(indent);
    match node {
        MinifiedNode::String(s) => {
            buf.push_str(&format!("{}* {}\n", spaces, s));
        }
        MinifiedNode::Array(arr) => {
            for sub in arr {
                stringify_list_item(buf, sub, indent + 2);
            }
        }
        _ => {}
    }
}
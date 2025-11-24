use crate::MinifiedNode;

/// Recursively traverses the AST and replaces keys with valid XML tag names
pub fn sanitize_keys(node: MinifiedNode) -> MinifiedNode {
    match node {
        MinifiedNode::Map(entries) => {
            let sanitized_entries = entries.into_iter().map(|(key, value)| {
                let clean_key = sanitize_xml_tag(&key);
                (clean_key, sanitize_keys(value))
            }).collect();
            MinifiedNode::Map(sanitized_entries)
        },
        MinifiedNode::Array(items) => {
            let sanitized_items = items.into_iter().map(sanitize_keys).collect();
            MinifiedNode::Array(sanitized_items)
        },
        // Leaves (Strings) are content, so they don't need sanitisation
        _ => node,
    }
}

/// Transforms a string into a valid XML tag
/// E.g., "My Key" -> "My_Key", "1st Item" -> "_1st_Item"
fn sanitize_xml_tag(s: &str) -> String {
    let mut out = String::with_capacity(s.len());

    for (i,c) in s.chars().enumerate() {
        if i == 0 {
            // XML tags must start with a letter or underscore
            if c.is_alphabetic() || c == '_' {
                out.push(c);
            } else {
                // Prefix with underscore if it starts with invalid char
                out.push('_');
                if c.is_alphanumeric() {
                    out.push(c);
                }
            }
        } else {
            // Subsequent chars can be alphanumeric
            if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' {
                out.push(c);
            } else {
                // Replace invalid chars with underscore
                out.push('_');
            }
        }
    }

    if out.is_empty() {
        return "_".to_string();
    }
    out
}
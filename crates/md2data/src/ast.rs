use serde::{Serialize, Serializer};
use serde::ser::SerializeMap;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Node {
    Document  { children: Vec<NodeOrString> },
    Heading   { level: u8, children: Vec<NodeOrString> },
    Paragraph { children: Vec<NodeOrString> },
    CodeBlock { info: Option<String>, content: String },
    List      { ordered: bool, items: Vec<Node> },
    ListItem  { children: Vec<NodeOrString> },
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum NodeOrString {
    String(String),
    Node(Box<Node>), // Box needed for recursion    
}

/// A simplified AST for the "Minified/Config" mode.
/// 
/// Features:
/// - Preserves key order
/// - Differentiates between Leaf (String), List (Array), and Branch (Map)
#[derive(Debug, Clone, PartialEq)]
pub enum MinifiedNode {
    String(String),
    Array(Vec<MinifiedNode>),
    // We use Vec<(Key, Value)> instead of HashMap to strictly preserve 
    // the order of keys as they appear in the Markdown.
    Map(Vec<(String, MinifiedNode)>),
}

// Custom serializer to make the Map variant output as a JSON object, not an array of tuples
impl Serialize for MinifiedNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MinifiedNode::String(s) => serializer.serialize_str(s),
            MinifiedNode::Array(arr) => arr.serialize(serializer),
            MinifiedNode::Map(kvs) => {
                let mut map = serializer.serialize_map(Some(kvs.len()))?;
                for (k, v) in kvs {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

use serde::Serialize;

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

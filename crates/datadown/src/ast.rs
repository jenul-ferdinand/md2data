use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Node {
    Document { children: Vec<Node> },
    Heading  { level: u8, text: String },
    Paragraph{ children: Vec<Node> },
    Text     { content: String },
    CodeBlock{ info: Option<String>, content: String },
    List     { ordered: bool, items: Vec<Node> },
    ListItem { children: Vec<Node> },
}

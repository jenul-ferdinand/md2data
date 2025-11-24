use md2data::parse_markdown;
use md2data::Node;
use md2data::NodeOrString;
use pretty_assertions::assert_eq;

fn unwrap_node(ns: &NodeOrString) -> &Node {
    match ns { 
        NodeOrString::Node(n) => n,
        _ => panic!("Expected Node, got String"),
    }
}
fn unwrap_text(ns: &NodeOrString) -> &str {
    match ns {
        NodeOrString::String(s) => s,
        _ => panic!("Expected String, got Node"),
    }
}

/// Simple H1 Test
/// 
/// INPUT
/// # Hello World
/// 
/// OUTPUT
/// Document:
///     Heading:
///         level: 1
///         children: 
///             "Hello World"
#[test]
fn test_simple_heading1() {
    let md = "# Hello World";
    let ast = parse_markdown(md);

    if let Node::Document { children } = ast {
        let heading_node = unwrap_node(&children[0]);

        if let Node::Heading { level, children } = heading_node {
            assert_eq!(*level, 1);
            assert_eq!(unwrap_text(&children[0]), "Hello World");
        } else {
            panic!("Not a Heading");
        }
    } else {
        panic!("Not a Document");
    }
}

/// Double heading test with H1 and H2
/// 
/// INPUT
/// # Foo
/// ## Bar
/// 
/// OUTPUT
/// Document:
///     Heading:
///         level: 1
///         children:
///             "Foo"
///     Heading:
///         level: 2
///         children:
///             "Bar"
#[test]
fn test_simple_heading1_2() {
    let md = "# Foo\n\n\n\n\n## Bar";
    let ast = parse_markdown(md);
    println!("{:?}", ast);

    if let Node::Document { children: document_children } = ast {
        let h1_node = unwrap_node(&document_children[0]);
        // Heading 1
        if let Node::Heading { level, children } = h1_node {
            assert_eq!(*level, 1);
            assert_eq!(unwrap_text(&children[0]), "Foo");
        } else {
            panic!("Not a Heading");
        }
        
        let h2_node = unwrap_node(&document_children[1]);
        // Heading 2
        if let Node::Heading { level, children } = h2_node {
            assert_eq!(*level, 2);
            assert_eq!(unwrap_text(&children[0]), "Bar");
        } else {
            panic!("Not a heading");
        }
    } else {
        panic!("Not a Document");
    }
}

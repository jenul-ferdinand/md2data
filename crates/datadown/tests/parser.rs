use datadown::parse_markdown;
use datadown::Node;
use datadown::NodeOrString;
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
#[test]
fn test_simple_heading1_2() {
    let md = "# Foo\n## Bar";
    let ast = parse_markdown(md);
    println!("{:?}", ast);

    if let Node::Document { children: doc_children } = ast {
        let h1_node = unwrap_node(&doc_children[0]);
        // Heading 1
        if let Node::Heading { level: h1_level, children: h1_children } = h1_node {
            assert_eq!(*h1_level, 1);
            assert_eq!(unwrap_text(&h1_children[0]), "Foo");
        } else {
            panic!("Not a Heading");
        }
        
        let h2_node = unwrap_node(&doc_children[1]);
        // Heading 2
        if let Node::Heading { level: h2_level, children: h2_children } = h2_node {
            assert_eq!(*h2_level, 2);
            assert_eq!(unwrap_text(&h2_children[0]), "Bar");
        } else {
            panic!("Not a heading");
        }
    } else {
        panic!("Not a Document");
    }
}

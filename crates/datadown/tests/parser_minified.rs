use datadown::{parse_markdown_minified, MinifiedNode};
use pretty_assertions::assert_eq;

// Helper to extract string value for easier assertions
fn unwrap_string(node: &MinifiedNode) -> &str {
    match node {
        MinifiedNode::String(s) => s,
        _ => panic!("Expected String, got {:?}", node),
    }
}

// Helper to extract array for easier assertions
fn unwrap_array(node: &MinifiedNode) -> &Vec<MinifiedNode> {
    match node {
        MinifiedNode::Array(arr) => arr,
        _ => panic!("Expected Array, got {:?}", node),
    }
}

// Helper to extract map for easier assertions
fn unwrap_map(node: &MinifiedNode) -> &Vec<(String, MinifiedNode)> {
    match node {
        MinifiedNode::Map(map) => map,
        _ => panic!("Expected Map, got {:?}", node),
    }
}

#[test]
fn test_minified_flat_key_value() {
    let md = r#"
# Name
Datadown Parser

# Version
1.0.0

# Active
true
"#;
    let ast = parse_markdown_minified(md);
    let map = unwrap_map(&ast);

    // Should have 3 keys
    assert_eq!(map.len(), 3);

    // Check "Name"
    assert_eq!(map[0].0, "Name");
    assert_eq!(unwrap_string(&map[0].1), "Datadown Parser");

    // Check "Version"
    assert_eq!(map[1].0, "Version");
    assert_eq!(unwrap_string(&map[1].1), "1.0.0");

    // Check "Active"
    assert_eq!(map[2].0, "Active");
    assert_eq!(unwrap_string(&map[2].1), "true");
}

#[test]
fn test_minified_lists() {
    let md = r#"
# Team
* Jenul
* Foo
* Bar

# Tags
* Rust
* CLI
"#;
    let ast = parse_markdown_minified(md);
    let map = unwrap_map(&ast);

    assert_eq!(map.len(), 2);

    // Check "Team" (Array)
    assert_eq!(map[0].0, "Team");
    let team_arr = unwrap_array(&map[0].1);
    assert_eq!(team_arr.len(), 3);
    assert_eq!(unwrap_string(&team_arr[0]), "Jenul");
    assert_eq!(unwrap_string(&team_arr[1]), "Foo");

    // Check "Tags" (Array)
    assert_eq!(map[1].0, "Tags");
    let tags_arr = unwrap_array(&map[1].1);
    assert_eq!(tags_arr.len(), 2);
    assert_eq!(unwrap_string(&tags_arr[0]), "Rust");
}

#[test]
fn test_minified_nested_objects() {
    let md = r#"
# Database
## Connection
localhost

## Port
5432

# Server
## Config
### Timeout
30s
"#;
    let ast = parse_markdown_minified(md);
    let root_map = unwrap_map(&ast);

    // 1. Database
    assert_eq!(root_map[0].0, "Database");
    let db_map = unwrap_map(&root_map[0].1);
    
    // Database -> Connection
    assert_eq!(db_map[0].0, "Connection");
    assert_eq!(unwrap_string(&db_map[0].1), "localhost");
    
    // Database -> Port
    assert_eq!(db_map[1].0, "Port");
    assert_eq!(unwrap_string(&db_map[1].1), "5432");

    // 2. Server
    assert_eq!(root_map[1].0, "Server");
    let server_map = unwrap_map(&root_map[1].1);
    
    // Server -> Config
    assert_eq!(server_map[0].0, "Config");
    let config_map = unwrap_map(&server_map[0].1);
    
    // Server -> Config -> Timeout
    assert_eq!(config_map[0].0, "Timeout");
    assert_eq!(unwrap_string(&config_map[0].1), "30s");
}

#[test]
fn test_minified_conflict_resolution() {
    // Test the rule: "Subheadings overwrite direct text"
    // logic: # A has text "Ignored Text", but then ## B appears.
    // The parser should convert A into a Map, clobbering "Ignored Text".
    let md = r#"
# A
Ignored Text

## B
Kept Text
"#;
    let ast = parse_markdown_minified(md);
    let root_map = unwrap_map(&ast);

    assert_eq!(root_map[0].0, "A");
    
    // A should be a Map, not a String
    let a_map = unwrap_map(&root_map[0].1);
    
    // It should contain B
    assert_eq!(a_map[0].0, "B");
    assert_eq!(unwrap_string(&a_map[0].1), "Kept Text");
}

#[test]
fn test_minified_nested_list_items() {
    // Test nested lists within lists (Array of Arrays)
    let md = r#"
# Matrix
* Row 1
  * Col 1
  * Col 2
* Row 2
  * Col 3
"#;
    let ast = parse_markdown_minified(md);
    let root_map = unwrap_map(&ast);
    
    assert_eq!(root_map[0].0, "Matrix");
    let matrix = unwrap_array(&root_map[0].1);
    
    // Item 1 should be an Array (the nested list)
    // Note: In our implementation, if an item has a nested list, 
    // the item becomes that list. The text "Row 1" is currently discarded 
    // or needs specific handling in parse_recursive_list if we want to keep it.
    // Based on current impl, it effectively becomes [ ["Col 1", "Col 2"], ["Col 3"] ]
    
    let row1 = unwrap_array(&matrix[0]);
    assert_eq!(unwrap_string(&row1[0]), "Col 1");
    assert_eq!(unwrap_string(&row1[1]), "Col 2");
    
    let row2 = unwrap_array(&matrix[1]);
    assert_eq!(unwrap_string(&row2[0]), "Col 3");
}
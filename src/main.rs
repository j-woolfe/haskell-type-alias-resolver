use tree_sitter::Node as TSNode;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use std::fs::read_to_string;
use std::path::Path;

extern "C" {
    fn tree_sitter_haskell() -> Language;
}

// fn get_representation<'a>(node: TSNode, source: &'a [u8]) -> String {
//     let mut cursor = node.walk();
//     let lhs_name = node
//         .child_by_field_name("name")
//         .unwrap()
//         .utf8_text(source)
//         .unwrap();

//     // TODO: Something with type variables

//     let rhs_name = node
//         .children(&mut cursor)
//         .find(|n| n.kind() == "type_name")
//         .unwrap()
//         .utf8_text(source)
//         .unwrap();

//     format!("{} := {}", lhs_name, rhs_name)
// }

struct Alias {
    query_str: String,
    source: String,
    terms: Vec<String>,
}

fn create_target_alias(in_sig: &[u8]) -> Alias {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let mut query_cursor = QueryCursor::new();

    // Input Type sig
    let sig_tree = parser.parse(in_sig, None).unwrap();

    let sig_query = "(signature) @sig";
    let get_sig_type = Query::new(language, &sig_query).unwrap();
    let sig_matches = query_cursor.matches(&get_sig_type, sig_tree.root_node(), in_sig);

    let sig_nodes: Vec<TSNode> = sig_matches
        .flat_map(|m| m.captures)
        .map(|m| {
            m.node
                .child_by_field_name("type")
                .unwrap()
                .next_sibling()
                .unwrap()
        })
        .collect();

    let query_str = format!("{}{}{}", "(type_alias ", sig_nodes[0].to_sexp(), ") @alias");
    let source = sig_nodes[0].utf8_text(in_sig).unwrap().to_string();
    // println!("{}", sig_nodes[0].to_sexp());

    let terms = get_terms(&sig_nodes[0], in_sig);

    Alias {
        query_str,
        source,
        terms,
    }
}

fn get_terms<'a>(node: &TSNode, source: &'a [u8]) -> Vec<String> {
    let mut cursor = node.walk();

    node.children(&mut cursor)
        .filter(|n| n.kind() == "type_name")
        .map(|n| n.utf8_text(source).unwrap().to_string())
        .collect()
}

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let mut query_cursor = QueryCursor::new();

    // Input Type sig
    let input_sig = "afunc :: (String, JValue)".as_bytes();

    let target_alias = create_target_alias(input_sig);

    println!("{}", target_alias.source);
    println!("{:?}", target_alias.terms);

    // let source_path = Path::new("test.hs");
    let source_path = Path::new("jpairExample.hs");
    let source_code = read_to_string(source_path).unwrap();
    let source = source_code.as_bytes();

    // let source = "type TestTuple a b = (a, b)".as_bytes();

    let tree = parser.parse(source, None).unwrap();

    // dbg!(&query);

    let get_type_aliases = Query::new(language, &target_alias.query_str).unwrap();

    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source);

    let nodes = matches
        .flat_map(|m| m.captures)
        .map(|m| m.node)
        // .inspect(|n| println!("{}", n.child(3).unwrap().to_sexp()))
        // .inspect(|n| println!("{:?}", get_terms(&n.child(3).unwrap(), source)))
        .filter(|n| get_terms(&n.child(3).unwrap(), source).eq(&target_alias.terms));
    // let filtered_nodes = nodes.filter(|n|
    // let strings = nodes.map(|n| get_representation(n, source));
    let strings = nodes.map(|n| n.utf8_text(source).unwrap());
    // let strings = nodes.map(|n| n.to_sexp());

    // println!("{}", tree.root_node().to_sexp());

    println!("Matching types:");
    for string in strings {
        println!("{}", string);
    }
}

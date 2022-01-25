use tree_sitter::Node as TSNode;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use std::fs::read_to_string;
use std::path::Path;

extern "C" {
    fn tree_sitter_haskell() -> Language;
}

fn get_representation<'a>(node: TSNode, source: &'a [u8]) -> String {
    let mut cursor = node.walk();
    let lhs_name = node
        .child_by_field_name("name")
        .unwrap()
        .utf8_text(source)
        .unwrap();

    // TODO: Something with type variables

    let rhs_name = node
        .children(&mut cursor)
        .find(|n| n.kind() == "type_name")
        .unwrap()
        .utf8_text(source)
        .unwrap();

    format!("{} := {}", lhs_name, rhs_name)
}

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let mut query_cursor = QueryCursor::new();

    // Input Type sig
    let input_sig = "afunc :: (a, b)";
    let sig_tree = parser.parse(input_sig.as_bytes(), None).unwrap();

    let sig_query = "(signature) @sig";
    let get_sig_type = Query::new(language, &sig_query).unwrap();
    let sig_matches = query_cursor.matches(&get_sig_type, sig_tree.root_node(), input_sig.as_bytes());

    println!("Input type signature");
    println!("{}", input_sig);
    // println!("{}", sig_tree.root_node().to_sexp());

    let sig_nodes: Vec<String> = sig_matches
        .flat_map(|m| m.captures)
        .map(|m| {
            m.node
                .child_by_field_name("type")
                .unwrap()
                .next_sibling()
                .unwrap()
                .to_sexp()
            // .utf8_text(input_sig)
            // .unwrap()
        })
        .inspect(|x| println!("{}", x))
        .collect();
    println!();

    let source_path = Path::new("test2.hs");
    let source_code = read_to_string(source_path).unwrap();
    let source = source_code.as_bytes();

    // let source = "type TestTuple a b = (a, b)".as_bytes();

    let tree = parser.parse(source, None).unwrap();

    let query = format!("{}{}{}", "(type_alias ", sig_nodes[0], ") @alias");
    // dbg!(&query);

    let get_type_aliases = Query::new(language, &query).unwrap();

    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source);

    let nodes = matches.flat_map(|m| m.captures).map(|m| m.node);
    // let strings = nodes.map(|n| get_representation(n, source));
    let strings = nodes.map(|n| n.utf8_text(source).unwrap());

    // println!("{}", tree.root_node().to_sexp());

    println!();

    for string in strings {
        println!("{}", string);
    }
}

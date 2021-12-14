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

    format!("{} => {}", lhs_name, rhs_name)
}

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let source_path = Path::new("test.hs");
    let source_code = read_to_string(source_path).unwrap();
    let source = source_code.as_bytes();

    // let source = "type TestTuple a b = (a, b)".as_bytes();
    // let source = "type TestVariable a = Maybe a".as_bytes();

    let tree = parser.parse(source, None).unwrap();

    let query = "(type_alias) @alias";

    // let type_aliases_sexp = "(type_alias name: (type) @alias_lhs (type_name (type) @alias_rhs))";
    // let type_list_sexp = "(type_alias name: (type) @list_lhs (type_list (type_name (type))) @list_rhs)";
    // let query = format!("{} {}", type_aliases_sexp, type_list_sexp);
    let get_type_aliases = Query::new(language, &query).unwrap();

    let mut query_cursor = QueryCursor::new();

    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source);

    let nodes = matches.flat_map(|m| m.captures).map(|m| m.node);
    let strings = nodes.map(|n| get_representation(n, source));

    println!("{}", tree.root_node().to_sexp());

    for string in strings {
        println!("{}", string);
    }
}

use tree_sitter::{Parser, Language, Query, QueryCursor};

use std::path::Path;
use std::fs::read_to_string;

extern "C" {fn tree_sitter_haskell() -> Language; }


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

    let all_matches = matches.flat_map(|m| m.captures);
    // let strings = all_matches.map(|m| m.node.to_sexp());
    let strings = all_matches.map(|m| m.node.child(1).unwrap().utf8_text(source).unwrap());


    // let new_type_name = tree.root_node().child(0).unwrap().child(1).unwrap().utf8_text(source).unwrap();

    println!("{}", tree.root_node().to_sexp());

    for string in strings {
        println!("{}", string);
        
    }
}


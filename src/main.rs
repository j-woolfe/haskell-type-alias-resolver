use tree_sitter::Node as TSNode;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use std::fs::read_to_string;

// CLI library
use clap::Parser as CLIParser;

// JSON Output
use serde::{Deserialize, Serialize};
use serde_json::json;

extern "C" {
    fn tree_sitter_haskell() -> Language;
}

#[derive(CLIParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    type_sig: String,

    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

struct Alias {
    query_str: String,
    source: String,
    terms: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct Match {
    matched: String,
    location: [[usize; 2]; 2],
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

    let query_str = format!("{}{}{}", "(type_alias ", sig_nodes[0].to_sexp(), " @alias)");
    println!("Query = {}", query_str);
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
    // let mut cursor = node.walk();

    // node.children(&mut cursor)
    //     .filter(|n| n.kind() == "type_name")
    //     .map(|n| n.utf8_text(source).unwrap().to_string())
    //     .collect()

    let mut query_cursor = QueryCursor::new();

    let type_query = "(type_name) @type";

    let language = unsafe { tree_sitter_haskell() };

    let get_types = Query::new(language, &type_query).unwrap();
    let type_matches = query_cursor.matches(&get_types, *node, source);

    type_matches
        .flat_map(|m| m.captures)
        .map(|m| m.node.utf8_text(source).unwrap().to_string())
        .collect()
}

fn main() {
    let args = Args::parse();

    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let mut query_cursor = QueryCursor::new();

    // Input Type sig
    let input_sig = format!("afunc :: {}", args.type_sig);
    // let input_sig = "afunc :: (String, JValue)".as_bytes();
    // let input_sig = "afunc :: Either Error Code".as_bytes();

    let target_alias = create_target_alias(input_sig.as_bytes());

    println!("{}", target_alias.source);
    println!("{:?}", target_alias.terms);

    // let source_path = Path::new("lockerLookupExample.hs");
    // let source_path = Path::new("jpairExample.hs");
    // let source_code = read_to_string(source_path).unwrap();
    let source_code = read_to_string(args.path).unwrap();
    let source = source_code.as_bytes();

    // let source = "type TestTuple a b = (a, b)".as_bytes();

    let tree = parser.parse(source, None).unwrap();

    // dbg!(&query);

    let get_type_aliases = Query::new(language, &target_alias.query_str).unwrap();

    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source);

    let nodes = matches
        .flat_map(|m| m.captures)
        .map(|m| m.node)
        // .inspect(|n| println!("{}", n.to_sexp()))
        // .inspect(|n| println!("{}", n.child(3).unwrap().to_sexp()))
        // .inspect(|n| println!("{:?}", get_terms(n, source)))
        .filter(|n| get_terms(n, source).eq(&target_alias.terms));
    // let strings = nodes.map(|n| n.parent().unwrap().utf8_text(source).unwrap());
    // let strings = nodes.map(|n| n.to_sexp());

    // println!("{}", tree.root_node().to_sexp());

    // println!("\nMatching types:");
    // for string in strings {
    //     println!("{}", string);
    // }

    println!("\nJSON Output");

    let out_matches: Vec<Match> = nodes
        .map(|n| Match {
            matched: n.parent().unwrap().utf8_text(source).unwrap().to_string(),
            location: [
                [n.start_position().row, n.start_position().column],
                [n.end_position().row, n.end_position().column],
            ],
        })
        .collect();

    let out = json!(
       {
        "input": {
            "type": args.type_sig,
            // "text": source_code
            "text": "OMITTED"
        },
        "output": out_matches
    });

    println!("{}", out.to_string());
}

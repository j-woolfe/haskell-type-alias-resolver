use tree_sitter::Node as TSNode;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use std::collections::HashMap;

// JSON Output
use serde::{Deserialize, Serialize};
use serde_json::json;

use regex::Regex;
extern "C" {
    fn tree_sitter_haskell() -> Language;
}

#[derive(Debug, PartialEq)]
enum Term {
    Type(String),
    Variable(String),
}

#[allow(dead_code)]
struct Alias {
    query_str: String,
    source: String,
    terms: Vec<Term>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestAlias {
    pub target_type: String,
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMatches {
    echo_request: RequestAlias,
    matches: Vec<Match>,
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
    // let sig_matches = query_cursor.matches(&get_sig_type, sig_tree.root_node(), in_sig);
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

    // let query_str = format!("{}{}{}", "(type_alias ", sig_nodes[0].to_sexp(), " @alias)");

    let query_str_pre = format!("{}{}{}", "(type_alias ", sig_nodes[0].to_sexp(), " @alias)");
    let re = Regex::new(r"\(type_variable\)|\(type\)").unwrap();
    let query_str = re
        .replace_all(&query_str_pre, "[(type) (type_variable)]")
        .to_string();

    // println!("Query = {}", query_str);
    let source = sig_nodes[0].utf8_text(in_sig).unwrap().to_string();
    // println!("{}", sig_nodes[0].to_sexp());

    let terms = get_terms(&sig_nodes[0], in_sig);

    Alias {
        query_str,
        source,
        terms,
    }
}

fn get_terms<'a>(node: &TSNode, source: &'a [u8]) -> Vec<Term> {
    let mut cursor = node.walk();

    // node.children(&mut cursor)
    //     .filter(|n| n.kind() == "type_name")
    //     .map(|n| n.utf8_text(source).unwrap().to_string())
    //     .collect()

    let mut query_cursor = QueryCursor::new();

    let type_query = "(type_name) @type";

    let language = unsafe { tree_sitter_haskell() };

    let get_types = Query::new(language, &type_query).unwrap();
    let type_matches = query_cursor.matches(&get_types, *node, source);

    // let out =
    type_matches
        .flat_map(|m| m.captures)
        .map(|m| {
            // if m.node.child_by_field_name("type_variable").is_none() {
            if m.node
                .children(&mut cursor)
                .any(|n| n.kind() == "type_variable")
            {
                Term::Variable(m.node.utf8_text(source).unwrap().to_string())
            } else {
                Term::Type(m.node.utf8_text(source).unwrap().to_string())
            }
        })
        .collect()

    // println!("{:?}", out);
    // out
}

fn check_variable_consistency(target_terms: &Vec<Term>, candidate_terms: &Vec<Term>) -> bool {
    let mut variable_map = HashMap::new();

    let pairs = candidate_terms.iter().zip(target_terms.iter());

    for pair in pairs {
        match pair {
            (Term::Variable(v), Term::Type(t)) => match variable_map.insert(v, t) {
                None => {}
                Some(old_t) => {
                    if old_t != t {
                        return false;
                    }
                }
            },
            (Term::Type(t1), Term::Type(t2)) => {
                if t1 != t2 {
                    return false;
                }
            }
            (_, Term::Variable(_)) => {
                panic!("Expected target alias to be concrete type")
            }
        }
    }

    true
}

pub fn alias_replacement(request: RequestAlias) -> ResponseMatches {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let mut query_cursor = QueryCursor::new();

    let source_bytes = request.source.as_bytes();

    // Input Type sig
    let input_sig = format!("afunc :: {}", request.target_type);

    let target_alias = create_target_alias(input_sig.as_bytes());

    let tree = parser.parse(&request.source, None).unwrap();

    let get_type_aliases = Query::new(language, &target_alias.query_str).unwrap();

    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source_bytes);

    let nodes = matches
        .flat_map(|m| m.captures)
        .map(|m| m.node)
        .filter(|n| check_variable_consistency(&target_alias.terms, &get_terms(n, source_bytes)));

    let out_matches: Vec<Match> = nodes
        .map(|n| Match {
            matched: n.parent().unwrap().utf8_text(source_bytes).unwrap().to_string(),
            location: [
                [n.start_position().row, n.start_position().column],
                [n.end_position().row, n.end_position().column],
            ],
        })
        .collect();

    ResponseMatches { 
        echo_request: request,
        matches: out_matches,
    }
    // json!(
    //    {
    //     "input": {
    //         "type": target_type,
    //         // "text": source_code
    //         "text": "OMITTED"
    //     },
    //     "output": out_matches
    // })
}

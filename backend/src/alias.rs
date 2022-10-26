use regex::Regex;

use std::collections::HashMap;

use crate::types::{Alias, Match, Position, Range, RequestAlias, ResponseMatches, Term};

// Treesitter
use tree_sitter::Node as TSNode;
use tree_sitter::{Language, Parser, Query, QueryCursor};
extern "C" {
    fn tree_sitter_haskell() -> Language;
}

fn create_target_alias(in_sig: &[u8]) -> Alias {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    let mut query_cursor = QueryCursor::new();

    // Input Type sig
    let sig_tree = parser.parse(in_sig, None).unwrap();

    let sig_query = Query::new(language, &"(signature) @sig").unwrap();
    let sig_matches = query_cursor.matches(&sig_query, sig_tree.root_node(), in_sig);

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

    // Generate query string matching a type or type variable wherever either appear
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
    let mut query_cursor = QueryCursor::new();

    let type_query = "(type_name) @type";

    let language = unsafe { tree_sitter_haskell() };

    let get_types = Query::new(language, &type_query).unwrap();
    let type_matches = query_cursor.matches(&get_types, *node, source);

    type_matches
        .flat_map(|m| m.captures)
        .map(|m| {
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
}

fn check_variable_consistency(
    target_terms: &Vec<Term>,
    candidate_terms: Vec<Term>,
) -> Option<HashMap<String, String>> {
    let mut variable_map: HashMap<String, String> = HashMap::new();

    let pairs = candidate_terms.iter().zip(target_terms.iter());

    for pair in pairs {
        match pair {
            (Term::Variable(v), Term::Type(t)) => {
                match variable_map.insert(v.to_string(), t.to_string()) {
                    None => {}
                    Some(old_t) => {
                        if old_t != *t {
                            return None;
                        }
                    }
                }
            }
            (Term::Type(t1), Term::Type(t2)) => {
                if t1 != t2 {
                    return None;
                }
            }
            (_, Term::Variable(_)) => {
                panic!("Expected target alias to be concrete type")
            }
        }
    }

    Some(variable_map)
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

    // let tree = parser.parse(&request.source, None).unwrap();
    let tree = parser.parse(&source_bytes, None).unwrap();

    let get_type_aliases = Query::new(language, &target_alias.query_str).unwrap();

    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source_bytes);

    let nodes = matches
        .flat_map(|m| m.captures)
        .map(|m| m.node)
        .map(|n| {
            (
                n, // Need to pass through node for replacement later
                check_variable_consistency(&target_alias.terms, get_terms(&n, source_bytes)),
            )
        })
        .filter(|(_, r)| r.is_some());

    let matches: Vec<Match> = nodes
        .map(|(n, r)| {
            let matched = n
                .parent()
                .unwrap()
                .utf8_text(source_bytes)
                .unwrap()
                .to_string();

            let location = Range {
                start: Position {
                    row: n.start_position().row,
                    col: n.start_position().column,
                },
                end: Position {
                    row: n.end_position().row,
                    col: n.end_position().column,
                },
            };

            let variable_map = r.unwrap();

            let re_name = Regex::new(r"type (.*)\s=").unwrap();
            let temp_matched = matched.clone();

            //TODO: Theres no way this is the way to do this
            let mut replaced_type = re_name
                .captures(&temp_matched)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();

            for (v, t) in variable_map.iter() {
                let re_str = format!(r" {}(?P<after> |\z)", v);
                let re_vars = Regex::new(&re_str).unwrap();
                replaced_type = re_vars
                    .replace_all(&replaced_type, format!(" {}$after", t))
                    .to_string()
            }

            Match {
                matched,
                location,
                variable_map,
                replaced_type: replaced_type.to_string(),
            }
        })
        .collect();

    ResponseMatches {
        echo_request: request,
        matches,
    }
}

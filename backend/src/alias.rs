// Contains the logic for using the Treesitter library to parse a source file into an AST
// and to query that AST for matching type aliases

use regex::Regex;
use std::collections::HashMap;

// Treesitter
use tree_sitter::Node as TSNode;
use tree_sitter::{Language, Parser, Query, QueryCursor};
extern "C" {
    fn tree_sitter_haskell() -> Language;
}

use crate::types::{Match, Position, Range, RequestAlias, ResponseMatches, Target, Term};

pub fn alias_replacement(request: RequestAlias) -> ResponseMatches {
    // Public API function to convert a request into a response
    // Details on RequestAlias and ResponseMatches can be found in types.rs

    // Initialise treesitter
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();
    let mut query_cursor = QueryCursor::new();

    // Source text
    let source_bytes = request.source.as_bytes();

    // Convert target to a Haskell type signature
    let input_sig = format!("afunc :: {}", request.target_type);

    // Convert type signature into a Treesitter query which matches type aliases with equivalent
    // types to the target
    let target_alias = create_target(input_sig.as_bytes());

    // Create AST
    let tree = parser.parse(&source_bytes, None).unwrap();

    // Create and run query
    let get_type_aliases = Query::new(language, &target_alias.query_str).unwrap();
    let matches = query_cursor.matches(&get_type_aliases, tree.root_node(), source_bytes);

    // Filter Treesitter captures to remove terms which have inconsistent type parameters
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

    // Process capture information from Treesitter into matches containing the required information
    // for response
    let matches: Vec<Match> = nodes
        .map(|(n, r)| {
            let matched = n
                .parent()
                .unwrap()
                .utf8_text(source_bytes)
                .unwrap()
                .to_string();

            // Location of matching type alias
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

            // A mapping from type variable to concrete type
            let variable_map = r.unwrap();

            // Extract name of alias
            // There is almost certainly a better way to do this but this works for now
            let temp_matched = matched.clone();
            let re_name = Regex::new(r"type (.*)\s=").unwrap();
            let mut replaced_type = re_name
                .captures(&temp_matched)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();

            // Substitute concrete types in for type variables
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

fn create_target(in_sig: &[u8]) -> Target {
    // Generates a Treesitter query which matches type aliases equivalent to `in_sig`

    // Initialise Treesitter
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();
    let mut query_cursor = QueryCursor::new();

    // Create Treesitter tree of input type
    // Note: "afunc :: " has been prepended to the target to make this possible
    let sig_tree = parser.parse(in_sig, None).unwrap();

    // Search for a type signature in the tree
    let sig_query = Query::new(language, &"(signature) @sig").unwrap();

    // Convert to a flat list of types. A matching alias must have the same structure of types as
    // this list
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

    // Reconstruct input signature
    let source = sig_nodes[0].utf8_text(in_sig).unwrap().to_string();

    // List of terms used to check variable consistency
    let terms = get_terms(&sig_nodes[0], in_sig);

    Target {
        query_str,
        source,
        terms,
    }
}

fn get_terms<'a>(node: &TSNode, source: &'a [u8]) -> Vec<Term> {
    // Transform a subtree from a treesitter node into a flat list of terms
    // A term is either a concrete type or a variable which will need to be
    // checked for consistency

    // Initialise Treesitter
    let mut cursor = node.walk();
    let mut query_cursor = QueryCursor::new();
    let language = unsafe { tree_sitter_haskell() };

    // Query for all type nodes below the parent
    let type_query = "(type_name) @type";
    let get_types = Query::new(language, &type_query).unwrap();
    let type_matches = query_cursor.matches(&get_types, *node, source);

    // Flatten and convert into Terms and return
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
    // Checks if the two lists of terms are consistent with eachother
    // If they are consistent, return a mapping from candidate type variables to target concrete
    // types
    // If not, return None

    // Initialise HashMap
    let mut variable_map: HashMap<String, String> = HashMap::new();

    let pairs = candidate_terms.iter().zip(target_terms.iter());

    for pair in pairs {
        match pair {
            // Check if a variable has already been assigned to a term
            // If not, assign a mapping
            // If so ensure the variable matches the type otherwise return None
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
            // Check if concrete types match
            (Term::Type(t1), Term::Type(t2)) => {
                if t1 != t2 {
                    return None;
                }
            }
            // At this stage, only concrete target types are supported
            (_, Term::Variable(_)) => {
                panic!("Expected target alias to be concrete type")
            }
        }
    }

    Some(variable_map)
}

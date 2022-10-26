// JSON Output
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Term {
    Type(String),
    Variable(String),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Alias {
    pub query_str: String,
    pub source: String,
    pub terms: Vec<Term>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestAlias {
    pub target_type: String,
    pub source: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMatches {
    pub echo_request: RequestAlias,
    pub matches: Vec<Match>,
}

impl fmt::Display for ResponseMatches {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let matches: Vec<String> = self
            .matches
            .clone()
            .into_iter()
            .map(|m| m.replaced_type)
            .collect();

        let target_type = self.echo_request.target_type.clone();

        let out_str = format!(
            "Target type: {}\nMatched:\n\t{}",
            target_type,
            matches.join("\n\t")
        );

        write!(f, "{}", out_str)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Match {
    pub matched: String,
    pub location: Range,
    pub variable_map: HashMap<String, String>,
    pub replaced_type: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

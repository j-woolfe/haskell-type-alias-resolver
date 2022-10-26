mod common;
use common::{TestCase, test_on_file};

#[test]
fn simple() {
    let test_case = TestCase {
        path: "simple.hs",
        target: "[Char]",
        result: vec!["String"],
    };
    test_on_file(test_case);
}

#[test]
fn simple_fail() {
    let test_case = TestCase {
        path: "simple.hs",
        target: "Int",
        result: vec![],
    };
    test_on_file(test_case);
}


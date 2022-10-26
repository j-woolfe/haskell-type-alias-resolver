// Tests matching concrete types with concrete targets

mod common;
use common::{TestCase, test_on_file};

#[test]
fn concrete_bool() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Bool",
        result: vec!["CBool"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_char() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Char",
        result: vec!["CChar"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_int() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Int",
        result: vec!["CInt"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_string() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "String",
        result: vec!["CString"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_void() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "()",
        result: vec!["CVoid"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_maybe() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Maybe String",
        result: vec!["CMaybe"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_list() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "[Int]",
        result: vec!["CList"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_1tuple() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(Char)",
        result: vec!["C1Tuple"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_2tuple() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(Int, Char)",
        result: vec!["C2Tuple"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_3tuple() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(String, Char, Bool)",
        result: vec!["C3Tuple"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_nested_list() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "[[Int]]",
        result: vec!["CNestedList"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_nested_tuple() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(String, (Int, Int))",
        result: vec!["CNestedTuple"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_nested_mixed() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "([Int], (Char, Bool))",
        result: vec!["CNestedMixed"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_function() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Bool -> Int",
        result: vec!["CFunction"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_function_list() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Int -> [String]",
        result: vec!["CFunctionList"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_function_tuple() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(Int, Int) -> Char",
        result: vec!["CFunctionTuple"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_function_nested() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(Int -> Char) -> [Int] -> [Char]",
        result: vec!["CFunctionNested"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_function_mixed() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "(Int -> (Char, String)) -> [Int] -> [(Char, String)]",
        result: vec!["CFunctionMixed"],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_fail() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "PathBuf",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_fail_complex() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "([Int], (Char, Bool, Bool))",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_fail_nested() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "[[[Int]]]",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn concrete_fail_function_nested() {
    let test_case = TestCase {
        path: "concrete.hs",
        target: "Int -> Char -> [Int] -> [Char]",
        result: vec![],
    };
    test_on_file(test_case);
}



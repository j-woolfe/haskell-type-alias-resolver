// Tests matching concrete types with generic targets

mod common;
use common::{test_on_file, TestCase};

#[test]
fn generic_tag() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "String",
        result: vec!["GTag String"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Int",
        result: vec!["GTag Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "TestType",
        result: vec!["GTag TestType"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_maybe() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "Maybe Char",
        result: vec!["GMaybe Char"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_list() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "[Bool]",
        result: vec!["GList Bool"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "[]",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn generic_void() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "()",
        result: vec!["GVoid"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_1tuple() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "(String)",
        result: vec!["G1Tuple String"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_2tuple() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "(Char, Int)",
        result: vec!["G2Tuple Char Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Int, Int)",
        result: vec!["G2Tuple Int Int", "G2TupleMatching Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Int, (String, String))",
        result: vec!["G2TupleNested String Int"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_list_tuples() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "[(Char, Bool)]",
        result: vec!["GListTuples Char Bool"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_function_bin() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "String -> String",
        result: vec!["GFunctionBin String", "GFunctionBinMixed String String"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_function_bin_mixed() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "String -> Int",
        result: vec!["GFunctionBinMixed String Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Int -> String",
        result: vec!["GFunctionBinMixed Int String"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_function_matching() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "Bool -> Bool -> Char",
        result: vec!["GFunctionMatching Bool Char"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Bool -> Char -> Char",
        result: vec![],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Bool -> Bool -> Bool",
        result: vec!["GFunctionMatching Bool Bool"],
    };
    test_on_file(test_case);
}

#[test]
fn generic_function_nested() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "(Char -> Char) -> Int",
        result: vec!["GFunctionNested Char Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Bool -> Bool) -> Bool",
        result: vec!["GFunctionNested Bool Bool"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Bool -> (Bool -> Bool)",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn generic_function_list() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "([Char] -> Char) -> Bool",
        result: vec!["GFunctionList Char Bool"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "([Int] -> Int) -> Int",
        result: vec!["GFunctionList Int Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Char -> [Char]) -> Int",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn generic_function_tuple() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "(Char, Int) -> (Char -> (Int, String))",
        result: vec!["GFunctionTuple Char Int String"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Bool, Bool) -> (Bool -> (Bool, Bool))",
        result: vec!["GFunctionTuple Bool Bool Bool"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Char -> (Char -> (Bool, String))",
        result: vec![],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Char, Char) -> (Char -> (Bool, String))",
        result: vec![],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "(Char, Bool) -> (Char -> (Char, String))",
        result: vec![],
    };
    test_on_file(test_case);
}

#[test]
fn generic_concrete_mixed() {
    let test_case = TestCase {
        path: "generic.hs",
        target: "Int -> String -> Int",
        result: vec!["GConcreteMixed Int"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "String -> String -> String",
        result: vec!["GConcreteMixed String", "GFunctionMatching String String"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "String -> Int -> String",
        result: vec![],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "String -> (Char -> Bool) -> String",
        result: vec!["GConcreteMixed2 String Char"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "Bool -> (Bool -> Bool) -> Bool",
        result: vec!["GConcreteMixed2 Bool Bool"],
    };
    test_on_file(test_case);

    let test_case = TestCase {
        path: "generic.hs",
        target: "String -> (Char, Bool) -> String",
        result: vec![],
    };
    test_on_file(test_case);
}

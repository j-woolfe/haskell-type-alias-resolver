use htar::run_on_file;

pub struct TestCase<'a> {
    pub path: &'a str,
    pub target: &'a str,
    pub result: Vec<&'a str>,
}

pub fn test_on_file(case: TestCase) {
    let full_path = format!("tests/input_files/{}", case.path);
    let response = run_on_file(full_path.into(), case.target.into());
    let mut replaced_types: Vec<String> = response
        .matches
        .into_iter()
        .map(|m| m.replaced_type)
        .collect();

    replaced_types.sort();

    assert!(
        replaced_types == case.result,
        "{}",
        format!("{:?} != {:?}", replaced_types, case.result)
    );
}

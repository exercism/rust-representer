use std::fs::File;
use std::io::Read;
use std::path::Path;

#[test]
fn test_single_let_binding() {
    let input_path = Path::new("./tests/test_input/single_let_binding.rs");
    let mut input_file = File::open(input_path).expect("Failed to open input file");

    let expected_path = Path::new("./tests/expected_output/single_let_binding.rs");
    let mut expected_file = File::open(expected_path).expect("Failed to open expected file");

    let mut input_src = String::new();
    let mut expected_src = String::new();

    input_file.read_to_string(&mut input_src).expect("Failed to read input file");
    expected_file.read_to_string(&mut expected_src).expect("Failed to read expected file");

    let replaced = representer::replace(&input_src).expect("Error in `replace`");

    assert_eq!(replaced.to_string(), expected_src);
}

#[test]
fn test_multiple_let_bindings() {
    let input_path = Path::new("./tests/test_input/multiple_let_bindings.rs");
    let mut input_file = File::open(input_path).expect("Failed to open input file");

    let expected_path = Path::new("./tests/expected_output/multiple_let_bindings.rs");
    let mut expected_file = File::open(expected_path).expect("Failed to open expected file");

    let mut input_src = String::new();
    let mut expected_src = String::new();

    input_file.read_to_string(&mut input_src).expect("Failed to read input file");
    expected_file.read_to_string(&mut expected_src).expect("Failed to read expected file");

    let replaced = representer::replace(&input_src).expect("Error in `replace`");

    assert_eq!(replaced.to_string(), expected_src);
}

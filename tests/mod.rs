use rust_representer::replace;
use std::error::Error;

// Set this to true to overwrite the expected output files.
static OVERWRITE_EXPECTED_OUTPUT: bool = false;

macro_rules! test_cases {
    ($($name: ident)*) => {
        $(
            #[test]
            fn $name() -> Result<(), Box<dyn Error>> {
                let input = include_str!(concat!(stringify!($name), "/src/lib.rs"));
                let expected = include_str!(concat!(stringify!($name), "/expected_representation.txt"));

                let mut input: syn::File = syn::parse_str(input)?;
                let mapping = replace(&mut input);

                if OVERWRITE_EXPECTED_OUTPUT {
                    std::fs::write(concat!("tests/", stringify!($name), "/expected_representation.txt"), prettyplease::unparse(&input))?;
                    std::fs::write(concat!("tests/", stringify!($name), "/expected_mapping.json"), serde_json::to_string_pretty(&mapping).unwrap())?;
                } else {
                    assert_eq!(prettyplease::unparse(&input), expected);
                }

                Ok(())
            }
        )*
    };
}

test_cases!(
    blocks
    closure_expressions
    const_names
    enum_names
    enum_variants
    fn_args
    fn_calls
    fn_names
    for_loops
    generics
    if_expressions
    if_let_expressions
    ignore_comments
    ignore_doc_comments
    impl_blocks
    leap_year
    let_bindings
    loops
    macro_inputs
    match_arms
    match_expressions
    method_calls
    replace_same_identifier
    sort_by_ascii_file_items
    static_names
    struct_fields
    struct_names
    type_aliases
    typed_let_binding
    union_names
    user_defined_traits
    user_defined_types
    while_loops
);

use rust_representer::replace;
use std::error::Error;

// Set this to true to overwrite the expected output files.
static OVERWRITE_EXPECTED_OUTPUT: bool = false;

macro_rules! test_cases {
    ($($name: ident)*) => {
        $(
            #[test]
            fn $name() -> Result<(), Box<dyn Error>> {
                let input = include_str!(concat!("test_input/", stringify!($name), ".rs"));
                let expected = include_str!(concat!("expected_output/", stringify!($name), ".rs"));

                let mut input: syn::File = syn::parse_str(input)?;
                let _ = replace(&mut input);

                if OVERWRITE_EXPECTED_OUTPUT {
                    std::fs::write(concat!("tests/expected_output/", stringify!($name), ".rs"), prettyplease::unparse(&input))?;
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
    if_expressions
    if_let_expressions
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

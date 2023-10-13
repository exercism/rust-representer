use rust_representer::replace;
use std::error::Error;
use std::include_str;

#[test]
fn test_let_bindings() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/let_bindings.rs");
    let expected = include_str!("expected_output/let_bindings.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_struct_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/struct_names.rs");
    let expected = include_str!("expected_output/struct_names.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_struct_fields() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/struct_fields.rs");
    let expected = include_str!("expected_output/struct_fields.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_enum_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/enum_names.rs");
    let expected = include_str!("expected_output/enum_names.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_enum_variants() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/enum_variants.rs");
    let expected = include_str!("expected_output/enum_variants.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_fn_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/fn_names.rs");
    let expected = include_str!("expected_output/fn_names.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_const_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/const_names.rs");
    let expected = include_str!("expected_output/const_names.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_static_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/static_names.rs");
    let expected = include_str!("expected_output/static_names.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_union_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/union_names.rs");
    let expected = include_str!("expected_output/union_names.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_type_aliases() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/type_aliases.rs");
    let expected = include_str!("expected_output/type_aliases.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_fn_args() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/fn_args.rs");
    let expected = include_str!("expected_output/fn_args.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_match_expressions() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/match_expressions.rs");
    let expected = include_str!("expected_output/match_expressions.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_match_arms() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/match_arms.rs");
    let expected = include_str!("expected_output/match_arms.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_macro_inputs() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/macro_inputs.rs");
    let expected = include_str!("expected_output/macro_inputs.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_fn_calls() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/fn_calls.rs");
    let expected = include_str!("expected_output/fn_calls.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_closure_expressions() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/closure_expressions.rs");
    let expected = include_str!("expected_output/closure_expressions.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_blocks() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/blocks.rs");
    let expected = include_str!("expected_output/blocks.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_for_loops() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/for_loops.rs");
    let expected = include_str!("expected_output/for_loops.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_method_calls() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/method_calls.rs");
    let expected = include_str!("expected_output/method_calls.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_loops() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/loops.rs");
    let expected = include_str!("expected_output/loops.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_while_loops() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/while_loops.rs");
    let expected = include_str!("expected_output/while_loops.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_if_expressions() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/if_expressions.rs");
    let expected = include_str!("expected_output/if_expressions.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_impl_blocks() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/impl_blocks.rs");
    let expected = include_str!("expected_output/impl_blocks.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_typed_let_binding() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/typed_let_binding.rs");
    let expected = include_str!("expected_output/typed_let_binding.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_ignore_comments() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/ignore_comments.rs");
    let expected = include_str!("expected_output/ignore_comments.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_ignore_doc_comments() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/ignore_doc_comments.rs");
    let expected = include_str!("expected_output/ignore_doc_comments.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_if_let_expressions() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/if_let_expressions.rs");
    let expected = include_str!("expected_output/if_let_expressions.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_user_defined_types() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/user_defined_types.rs");
    let expected = include_str!("expected_output/user_defined_types.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_user_defined_traits() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/user_defined_traits.rs");
    let expected = include_str!("expected_output/user_defined_traits.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_sort_by_ascii_file_items() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/sort_by_ascii_file_items.rs");
    let expected = include_str!("expected_output/sort_by_ascii_file_items.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_generics() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/generics.rs");
    let expected = include_str!("expected_output/generics.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

#[test]
fn test_leap_year() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/leap_year.rs");
    let expected = include_str!("expected_output/leap_year.rs");

    let mut input: syn::File = syn::parse_str(input)?;
    let _ = replace(&mut input);
    assert_eq!(prettyplease::unparse(&input), expected);

    Ok(())
}

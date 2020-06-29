use std::error::Error;
use std::include_str;

#[test]
fn test_single_let_binding() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/single_let_binding.rs");
    let expected = include_str!("expected_output/single_let_binding.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_multiple_let_bindings() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/multiple_let_bindings.rs");
    let expected = include_str!("expected_output/multiple_let_bindings.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_struct_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/struct_names.rs");
    let expected = include_str!("expected_output/struct_names.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_struct_fields() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/struct_fields.rs");
    let expected = include_str!("expected_output/struct_fields.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_enum_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/enum_names.rs");
    let expected = include_str!("expected_output/enum_names.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_enum_variants() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/enum_variants.rs");
    let expected = include_str!("expected_output/enum_variants.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_fn_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/fn_names.rs");
    let expected = include_str!("expected_output/fn_names.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_const_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/const_names.rs");
    let expected = include_str!("expected_output/const_names.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_static_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/static_names.rs");
    let expected = include_str!("expected_output/static_names.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_union_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/union_names.rs");
    let expected = include_str!("expected_output/union_names.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_type_aliases() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/type_aliases.rs");
    let expected = include_str!("expected_output/type_aliases.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_fn_args() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/fn_args.rs");
    let expected = include_str!("expected_output/fn_args.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_match_expressions() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/match_expressions.rs");
    let expected = include_str!("expected_output/match_expressions.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_match_arms() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/match_arms.rs");
    let expected = include_str!("expected_output/match_arms.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_macro_inputs() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/macro_inputs.rs");
    let expected = include_str!("expected_output/macro_inputs.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_fn_calls() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/fn_calls.rs");
    let expected = include_str!("expected_output/fn_calls.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_closure_expressions() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/closure_expressions.rs");
    let expected = include_str!("expected_output/closure_expressions.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_blocks() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/blocks.rs");
    let expected = include_str!("expected_output/blocks.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_for_loops() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/for_loops.rs");
    let expected = include_str!("expected_output/for_loops.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

#[test]
fn test_method_calls() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/method_calls.rs");
    let expected = include_str!("expected_output/method_calls.rs");

    let replaced = representer::replace(&input)?;
    assert_eq!(replaced.to_string(), expected);

    Ok(())
}

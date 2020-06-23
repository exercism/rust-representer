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
fn test_enum_names() -> Result<(), Box<dyn Error>> {
    let input = include_str!("test_input/enum_names.rs");
    let expected = include_str!("expected_output/enum_names.rs");

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

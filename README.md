# Rust Representer 

A normalizing representer for Exercism's Rust track.

## What's a Normalizing Representer?

A representer's job is to normalize some input code by stripping out and replacing any trivial details that introduce differences between students' submitted code. Comments, whitespace, and variable names, things that don't contribute to the overall logical flow and structure of the students' approach, are stripped out. In the case of variable names, these are replaced by a standard placeholder.

## Example

Given an example submission for the `two-fer` exercise like the following:

```rust
fn twofer(name: &str) -> String {
    match name {
        "" => "One for you, one for me.".to_string(),
        // use the `format!` macro to return a formatted String
        _ => format!("One for {}, one for me.", name),
    }
}
```

The representer will return:

```rust
fn PLACEHOLDER_1(PLACEHOLDER_2: &str) -> String {
    match PLACEHOLDER_2 {
        "" => "One for you, one for me.".to_string(),
        _ => format!("One for {}, one for me.", PLACEHOLDER_2),
    }
}
```

The ultimate purpose of the representer is to facilitate quicker response times from mentors by standardizing student implementations so that mentors can provide feedback on the _approach_ the student took to solve the problem. 

## Progress

Currently the following statements/expression types are visited by the representer:

- [x] `let` bindings
- [x] `struct` names 
- [x] `struct` fields
- [x] `enum` names 
- [x] `enum` variants
- [x] `fn` definitions
- [x] `fn` calls
- [ ] method calls
- [x] `const` names 
- [x] `static` names
- [x] `union` names
- [x] `type` aliases
- [x] `match` expressions
- [x] `match` arms
- [x] `macro` arguments
- [x] closure expressions
- [ ] loop expressions
- [ ] tuple expressions

## Design

The high-level steps the representer takes are as follows:

1. It first makes a formatting pass over the code using `rustfmt` in order to unsure whitespace and spacing are consistent in the input source code.
2. It then transforms the source code into an AST, stripping out comments in the process. 
3. From there, it traverses the AST, looking for identifiers.
4. When it finds an identifier:
    - It checks whether the identifier is a Rust keyword (or any other sort of identifier that isn't actually being used as a variable/function name).
    - If the identifier isn't a keyword, it then checks if the identifier is one that has been encountered before.
        - If it is, then a placeholder for this identifier has already been generated and stored in a HashMap; the identifier is replaced with the placeholder.
        - If it isn't, then the placeholder needs to be generated and saved in the HashMap before the identifier is replaced by it.
5. The transformed output is then put through another formatting pass. 

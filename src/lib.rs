#[macro_use]
extern crate lazy_static;

mod ident_visitor;
mod replace_identifier;
mod visit_mut;

use ident_visitor::IdentVisitor;
use serde_json::json;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use syn::visit_mut::VisitMut;

const PLACEHOLDER: &str = "PLACEHOLDER_";

const OUTPUT_FILE: &str = "representation.txt";
const MAPPINGS_FILE: &str = "mapping.json";
const REPRESENTATION_FILE: &str = "representation.json";

// The entry point that kicks off the process of visiting the AST
pub fn replace(ast: &mut syn::File) -> HashMap<String, String> {
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(ast);

    visitor
        .mappings
        .into_iter()
        .map(|(k, v)| (format!("{}{}", PLACEHOLDER, v), k))
        .collect()
}

pub fn run(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(input_path)?;
    let mut src = String::new();
    input.read_to_string(&mut src)?;

    let mut ast: syn::File = syn::parse_file(&src)?;
    let mappings = replace(&mut ast);

    let mut output = File::create(format!("{}{}", output_path, OUTPUT_FILE))?;
    let formatted = prettyplease::unparse(&ast);
    output.write_all(formatted.to_string().as_bytes())?;

    let mut output_mappings = File::create(format!("{}{}", output_path, MAPPINGS_FILE))?;
    output_mappings.write_all(serde_json::to_string_pretty(&mappings)?.as_bytes())?;

    let mut output_representation =
        File::create(format!("{}{}", output_path, REPRESENTATION_FILE))?;
    output_representation
        .write_all(serde_json::to_string_pretty(&json!({"version": 1}))?.as_bytes())?;
    Ok(())
}

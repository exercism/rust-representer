#[macro_use]
extern crate lazy_static;

mod ident_visitor;
mod replace_identifier;
mod visit_mut;

use ident_visitor::IdentVisitor;
use proc_macro2::TokenStream;
use quote::quote;
use std::fs::File;
use std::io::prelude::*;
use syn::visit_mut::VisitMut;

const OUTPUT_FILE: &str = "representation.rs";

// The entry point that kicks off the process of visiting the AST
pub fn replace(ast: &mut syn::File) -> TokenStream {
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(ast);

    quote!(#ast)
}

pub fn run(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = File::open(path)?;
    let mut src = String::new();
    input.read_to_string(&mut src)?;

    let mut ast: syn::File = syn::parse_file(&src)?;
    let replaced = replace(&mut ast);

    let mut output = File::create(format!("{}{}", path, OUTPUT_FILE))?;
    output.write_all(replaced.to_string().as_bytes())?;

    Ok(())
}

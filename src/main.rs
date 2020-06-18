use std::env;
use std::process;
use std::fs::File;
use std::io::Read;

use quote::quote;
use proc_macro2::{Ident, Span};
use syn::visit_mut::{self, VisitMut};
use syn::PatIdent;

const PLACEHOLDER: &str = "PLACEHOLDER";

struct LetBindingReplace;

impl VisitMut for LetBindingReplace {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        *node = PatIdent {
            attrs: vec![],
            by_ref: node.by_ref,
            mutability: node.mutability,
            ident: Ident::new(PLACEHOLDER, Span::call_site()),
            subpat: None,
        };
        
        visit_mut::visit_pat_ident_mut(self, node);
    }
}

fn main() {
    let mut args = env::args();
    let _ = args.next();

    let filename = match (args.next(), args.next()) {
        (Some(filename), None) => filename, 
        _ => {
            eprintln!("Usage: representer path/to/filename.rs");
            process::exit(1);
        }
    };

    let mut file = File::open(&filename).expect("Unable to open file");
    let mut src = String::new();
    
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut syntax_tree: syn::File = syn::parse_file(&src).expect("Unable to parse file");
    LetBindingReplace.visit_file_mut(&mut syntax_tree);
    println!("{}", quote!(#syntax_tree));
}

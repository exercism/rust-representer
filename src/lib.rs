use quote::quote;
use syn::PatIdent;
use proc_macro2::{Ident, Span};
use syn::visit_mut::{self, VisitMut};

use std::collections::HashMap;
use std::collections::hash_map::Entry;

const PLACEHOLDER: &str = "PLACEHOLDER_";

pub struct IdentVisitor {
    mappings: HashMap<String, u32>,
    uid: u32,
}

impl IdentVisitor {
    pub fn new() -> Self {
        IdentVisitor {
            mappings: HashMap::new(),
            uid: 0,
        }
    }
}

impl VisitMut for IdentVisitor {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        let ident_string = node.ident.to_string();
        let ph_num = match self.mappings.entry(ident_string) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                self.uid += 1;
                v.insert(self.uid)
            }
        };
        let ph_string = format!("{}{}", PLACEHOLDER, ph_num);

        *node = PatIdent {
            attrs: vec![],
            by_ref: node.by_ref,
            mutability: node.mutability,
            ident: Ident::new(&ph_string, Span::call_site()),
            subpat: None,
        };

        visit_mut::visit_pat_ident_mut(self, node);
    }
}

pub fn replace(src: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut syntax_tree: syn::File = syn::parse_file(&src)?;
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(&mut syntax_tree);
    println!("{}", quote!(#syntax_tree));

    Ok(())
}

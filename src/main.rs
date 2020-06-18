use quote::quote;
use proc_macro2::{Ident, Span};
use syn::visit_mut::{self, VisitMut};
use syn::{File, PatIdent};

struct LetBindingReplace;

impl VisitMut for LetBindingReplace {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        *node = PatIdent {
            attrs: vec![],
            by_ref: node.by_ref,
            mutability: node.mutability,
            ident: Ident::new("PLACEHOLDER", Span::call_site()),
            subpat: None,
        };
        
        visit_mut::visit_pat_ident_mut(self, node);
    }
}

fn main() {
    let code = quote! {
        fn main() {
            let x = 5;
            let _ = 999u256;
        }
    };

    let mut syntax_tree: File = syn::parse2(code).unwrap();
    LetBindingReplace.visit_file_mut(&mut syntax_tree);
    println!("{}", quote!(#syntax_tree));
}

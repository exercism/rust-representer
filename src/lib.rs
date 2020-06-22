use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::{ItemConst, ItemEnum, ItemStruct, PatIdent, Signature};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

const PLACEHOLDER: &str = "PLACEHOLDER_";
const KEYWORDS: &'static [&'static str] = &["main"];

pub struct IdentVisitor {
    mappings: HashMap<String, u32>,
    keywords: HashSet<&'static str>,
    uid: u32,
}

impl IdentVisitor {
    pub fn new() -> Self {
        IdentVisitor {
            mappings: HashMap::new(),
            keywords: KEYWORDS.iter().cloned().collect(),
            uid: 0,
        }
    }

    pub fn get_mapping(&mut self, ident: String) -> String {
        let uid = match self.mappings.entry(ident) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                self.uid += 1;
                v.insert(self.uid)
            }
        };

        format!("{}{}", PLACEHOLDER, uid)
    }
}

impl VisitMut for IdentVisitor {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let identifier = self.get_mapping(ident_string);

            node.ident = Ident::new(&identifier, Span::call_site());
        }
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let identifier = self.get_mapping(ident_string);

            node.ident = Ident::new(&identifier, Span::call_site());
        }
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let identifier = self.get_mapping(ident_string); 

            node.ident = Ident::new(&identifier, Span::call_site());
        }
    }

    fn visit_signature_mut(&mut self, node: &mut Signature) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let identifier = self.get_mapping(ident_string);

            node.ident = Ident::new(&identifier, Span::call_site());
        }
    }

    fn visit_item_const_mut(&mut self, node: &mut ItemConst) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let identifier = self.get_mapping(ident_string);

            node.ident = Ident::new(&identifier, Span::call_site());
        }
    }
}

pub fn replace(src: &str) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut syntax_tree: syn::File = syn::parse_file(&src)?;
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(&mut syntax_tree);

    Ok(quote!(#syntax_tree))
}

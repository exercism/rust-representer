use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::visit_mut::{self, VisitMut};
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
}

impl VisitMut for IdentVisitor {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
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
                subpat: node.subpat.clone(),
            };
        }

        visit_mut::visit_pat_ident_mut(self, node);
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let ph_num = match self.mappings.entry(ident_string) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => {
                    self.uid += 1;
                    v.insert(self.uid)
                }
            };
            let ph_string = format!("{}{}", PLACEHOLDER, ph_num);

            *node = ItemStruct {
                attrs: vec![],
                vis: node.vis.clone(),
                struct_token: node.struct_token,
                ident: Ident::new(&ph_string, Span::call_site()),
                generics: node.generics.clone(),
                fields: node.fields.clone(),
                semi_token: node.semi_token,
            };
        }

        visit_mut::visit_item_struct_mut(self, node);
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let ph_num = match self.mappings.entry(ident_string) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => {
                    self.uid += 1;
                    v.insert(self.uid)
                }
            };
            let ph_string = format!("{}{}", PLACEHOLDER, ph_num);

            *node = ItemEnum {
                attrs: vec![],
                vis: node.vis.clone(),
                enum_token: node.enum_token,
                ident: Ident::new(&ph_string, Span::call_site()),
                generics: node.generics.clone(),
                brace_token: node.brace_token,
                variants: node.variants.clone(),
            };
        }

        visit_mut::visit_item_enum_mut(self, node);
    }

    fn visit_signature_mut(&mut self, node: &mut Signature) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let ph_num = match self.mappings.entry(ident_string) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => {
                    self.uid += 1;
                    v.insert(self.uid)
                }
            };
            let ph_string = format!("{}{}", PLACEHOLDER, ph_num);

            *node = Signature {
                constness: node.constness,
                asyncness: node.asyncness,
                unsafety: node.unsafety,
                abi: node.abi.clone(),
                fn_token: node.fn_token,
                ident: Ident::new(&ph_string, Span::call_site()),
                generics: node.generics.clone(),
                paren_token: node.paren_token,
                inputs: node.inputs.clone(),
                variadic: node.variadic.clone(),
                output: node.output.clone(),
            };
        }

        visit_mut::visit_signature_mut(self, node);
    }

    fn visit_item_const_mut(&mut self, node: &mut ItemConst) {
        let ident_string = node.ident.to_string();

        if !self.keywords.contains::<str>(&ident_string) {
            let ph_num = match self.mappings.entry(ident_string) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => {
                    self.uid += 1;
                    v.insert(self.uid)
                }
            };
            let ph_string = format!("{}{}", PLACEHOLDER, ph_num);

            *node = ItemConst {
                attrs: vec![],
                vis: node.vis.clone(),
                const_token: node.const_token,
                ident: Ident::new(&ph_string, Span::call_site()),
                colon_token: node.colon_token,
                ty: node.ty.clone(),
                eq_token: node.eq_token,
                expr: node.expr.clone(),
                semi_token: node.semi_token,
            };
        }

        visit_mut::visit_item_const_mut(self, node);
    }
}

pub fn replace(src: &str) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut syntax_tree: syn::File = syn::parse_file(&src)?;
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(&mut syntax_tree);

    Ok(quote!(#syntax_tree))
}

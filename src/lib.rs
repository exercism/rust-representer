#[macro_use]
extern crate lazy_static;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::{FnArg, ItemConst, ItemEnum, ItemStatic, ItemStruct, ItemType, ItemUnion, Pat, PatIdent, PatType, Signature};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

const PLACEHOLDER: &str = "PLACEHOLDER_";

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("main");
        s
    };
}

pub trait ReplaceIdentifier {
    fn ident_string(&self) -> String;
    fn set_ident(&mut self, ident: String);
}

impl ReplaceIdentifier for PatIdent {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ItemStruct {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ItemEnum {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for Signature {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ItemConst {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ItemStatic {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ItemUnion {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ItemType {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

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

    pub fn visit_node<Node: ReplaceIdentifier>(&mut self, node: &mut Node) {
        let ident_string = node.ident_string();

        if !KEYWORDS.contains::<str>(&ident_string) {
            let identifier = self.get_mapping(ident_string);

            node.set_ident(identifier);
        }
    }
}

impl VisitMut for IdentVisitor {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        self.visit_node(node);
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        self.visit_node(node);
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        self.visit_node(node);
    }

    fn visit_signature_mut(&mut self, node: &mut Signature) {
        // handle Signature's identifier
        self.visit_node(node);

        // handle Signature's inputs, i.e. the function's arguments
        for node in node.inputs.iter_mut() {
            self.visit_fn_arg_mut(node);
        }
    }

    fn visit_item_const_mut(&mut self, node: &mut ItemConst) {
        self.visit_node(node);
    }

    fn visit_item_static_mut(&mut self, node: &mut ItemStatic) {
        self.visit_node(node);
    }

    fn visit_item_union_mut(&mut self, node: &mut ItemUnion) {
        self.visit_node(node);
    }

    fn visit_item_type_mut(&mut self, node: &mut ItemType) {
        self.visit_node(node);
    }

    fn visit_pat_mut(&mut self, node: &mut Pat) {
        if let Pat::Ident(pat_ident) = node {
            self.visit_pat_ident_mut(pat_ident);
        }
    }

    fn visit_pat_type_mut(&mut self, node: &mut PatType) {
        self.visit_pat_mut(&mut *node.pat);
    }

    fn visit_fn_arg_mut(&mut self, node: &mut FnArg) {
        if let FnArg::Typed(pat_type) = node {
            self.visit_pat_type_mut(pat_type);
        }
    }
}

pub fn replace(src: &str) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut syntax_tree: syn::File = syn::parse_file(&src)?;
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(&mut syntax_tree);

    Ok(quote!(#syntax_tree))
}

#[macro_use]
extern crate lazy_static;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::visit_mut::VisitMut;
use syn::{
    Arm, Expr, ExprMatch, ExprPath, FnArg, ItemConst, ItemEnum, ItemStatic, ItemStruct, ItemType,
    ItemUnion, Macro, Pat, PatIdent, PatTuple, PatType, Path, PathSegment, Signature,
};

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

const PLACEHOLDER: &str = "PLACEHOLDER_";

lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("main");
        s.insert("Some");
        s.insert("None");
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

impl ReplaceIdentifier for PathSegment {
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
        match node {
            Pat::Ident(pat_ident) => self.visit_pat_ident_mut(pat_ident),
            Pat::Tuple(pat_tuple) => self.visit_pat_tuple_mut(pat_tuple),
            Pat::TupleStruct(pat_tuple_struct) => self.visit_pat_tuple_struct_mut(pat_tuple_struct),
            _ => {}
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

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        match node {
            Expr::Match(expr_match) => self.visit_expr_match_mut(expr_match),
            Expr::Path(expr_path) => self.visit_expr_path_mut(expr_path),
            Expr::Macro(expr_macro) => self.visit_macro_mut(&mut expr_macro.mac),
            _ => {}
        }
    }

    fn visit_expr_match_mut(&mut self, node: &mut ExprMatch) {
        // visit the match expression
        self.visit_expr_mut(&mut *node.expr);

        // visit the match's arms
        for arm in node.arms.iter_mut() {
            self.visit_arm_mut(arm);
        }
    }

    fn visit_expr_path_mut(&mut self, node: &mut ExprPath) {
        self.visit_path_mut(&mut node.path);
    }

    fn visit_path_mut(&mut self, node: &mut Path) {
        if let Some(_) = node.get_ident() {
            // this is fine since we know `node` only contains a
            // single PathSegment which is an identifier
            self.visit_path_segment_mut(node.segments.first_mut().unwrap());
        }
    }

    fn visit_path_segment_mut(&mut self, node: &mut PathSegment) {
        self.visit_node(node);
    }

    fn visit_pat_tuple_mut(&mut self, node: &mut PatTuple) {
        for elem in node.elems.iter_mut() {
            self.visit_pat_mut(elem);
        }
    }

    fn visit_arm_mut(&mut self, node: &mut Arm) {
        // visit `Arm`'s expression
        self.visit_pat_mut(&mut node.pat);

        // visit `Arm`'s guard if it exists
        if let Some((_, expr)) = &mut node.guard {
            self.visit_expr_mut(expr);
        }

        // visit `Arm`'s body
        self.visit_expr_mut(&mut node.body);
    }

    fn visit_macro_mut(&mut self, node: &mut Macro) {
        unimplemented!();
    }
}

pub fn replace(src: &str) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut syntax_tree: syn::File = syn::parse_file(&src)?;
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(&mut syntax_tree);

    Ok(quote!(#syntax_tree))
}

#[macro_use]
extern crate lazy_static;

mod ident_visitor;
mod replace_identifier;

use proc_macro2::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{
    Arm, Expr, ExprBinary, ExprCall, ExprClosure, ExprMatch, ExprPath, FnArg, ItemConst, ItemEnum,
    ItemStatic, ItemStruct, ItemType, ItemUnion, Macro, Pat, PatIdent, PatTuple, PatType, Path,
    PathSegment, Signature, Token,
};

use ident_visitor::IdentVisitor;

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
            Expr::Call(expr_call) => self.visit_expr_call_mut(expr_call),
            Expr::Closure(expr_closure) => self.visit_expr_closure_mut(expr_closure),
            Expr::Binary(expr_binary) => self.visit_expr_binary_mut(expr_binary),
            Expr::Block(expr_block) => self.visit_expr_block_mut(expr_block),
            _ => {}
        }
    }

    fn visit_expr_binary_mut(&mut self, node: &mut ExprBinary) {
        self.visit_expr_mut(&mut *node.left);
        self.visit_expr_mut(&mut *node.right);
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

    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        self.visit_expr_mut(&mut *node.func);

        for arg in node.args.iter_mut() {
            self.visit_expr_mut(arg);
        }
    }

    fn visit_expr_closure_mut(&mut self, node: &mut ExprClosure) {
        for input in node.inputs.iter_mut() {
            self.visit_pat_mut(input);
        }

        self.visit_expr_mut(&mut *node.body);
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
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;

        if let Ok(ref mut body) = node.parse_body_with(parser) {
            for p in body.iter_mut() {
                self.visit_expr_mut(p);
            }

            node.tokens = quote!(#body);
        }
    }
}

pub fn replace(src: &str) -> Result<TokenStream, Box<dyn std::error::Error>> {
    let mut syntax_tree: syn::File = syn::parse_file(&src)?;
    let mut visitor = IdentVisitor::new();
    visitor.visit_file_mut(&mut syntax_tree);

    Ok(quote!(#syntax_tree))
}

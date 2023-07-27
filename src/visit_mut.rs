use quote::quote;
use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{
    Arm, Expr, ExprAssign, ExprBinary, ExprCall, ExprClosure, ExprField, ExprForLoop, ExprIf,
    ExprLet, ExprLoop, ExprMatch, ExprMethodCall, ExprPath, ExprUnary, ExprWhile, Field, Fields,
    FnArg, ItemConst, ItemEnum, ItemFn, ItemImpl, ItemStatic, ItemStruct, ItemTrait, ItemType,
    ItemUnion, Macro, Member, Pat, PatIdent, PatTuple, PatType, Path, PathSegment, ReturnType,
    Signature, Token, Type, Variant,
};

use crate::ident_visitor::IdentVisitor;

impl VisitMut for IdentVisitor {
    fn visit_pat_ident_mut(&mut self, node: &mut PatIdent) {
        self.replace_identifier(node);
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        // visit struct's identifier
        self.replace_identifier(node);

        // visit struct's fields
        for field in node.fields.iter_mut() {
            self.visit_field_mut(field);
        }
    }

    fn visit_item_fn_mut(&mut self, node: &mut ItemFn) {
        // clearing out the node's attributes is a hacky way
        // of ensuring `syn` doesn't parse doc comments
        node.attrs = vec![];

        // visit function's signature
        self.visit_signature_mut(&mut node.sig);

        // visit function's block
        self.visit_block_mut(&mut node.block);
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        // visit enum's identifier
        self.replace_identifier(node);

        // visit enum's variants
        for variant in node.variants.iter_mut() {
            self.visit_variant_mut(variant);
        }
    }

    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        // visit the impl's trait if it has one
        if let Some((_, ref mut path, _)) = node.trait_ {
            // only replace the trait's identifier if the trait is a
            // user-defined trait
            for segment in path.segments.iter_mut() {
                self.replace_identifier_if_mapped(segment);
            }
        }

        // visit the impl's type
        self.visit_type_mut(&mut node.self_ty);

        // visit the impl's items
        for impl_item in node.items.iter_mut() {
            self.visit_impl_item_mut(impl_item);
        }
    }

    fn visit_item_trait_mut(&mut self, node: &mut ItemTrait) {
        self.replace_identifier(node);

        // visit trait's items
        for trait_item in node.items.iter_mut() {
            self.visit_trait_item_mut(trait_item);
        }
    }

    fn visit_item_const_mut(&mut self, node: &mut ItemConst) {
        self.replace_identifier(node);
    }

    fn visit_item_static_mut(&mut self, node: &mut ItemStatic) {
        self.replace_identifier(node);
    }

    fn visit_item_union_mut(&mut self, node: &mut ItemUnion) {
        self.replace_identifier(node);
    }

    fn visit_item_type_mut(&mut self, node: &mut ItemType) {
        self.replace_identifier(node);
    }

    fn visit_signature_mut(&mut self, node: &mut Signature) {
        // handle signature's identifier
        self.replace_identifier(node);

        // handle signature's inputs, i.e. the function's arguments
        for node in node.inputs.iter_mut() {
            self.visit_fn_arg_mut(node);
        }

        // handle function's return type
        self.visit_return_type_mut(&mut node.output);
    }

    fn visit_return_type_mut(&mut self, node: &mut ReturnType) {
        if let ReturnType::Type(_, rt) = node {
            self.visit_type_mut(&mut *rt);
        }
    }

    fn visit_variant_mut(&mut self, node: &mut Variant) {
        // visit variant's identifier
        self.replace_identifier(node);

        // visit variant's fields
        self.visit_fields_mut(&mut node.fields);

        // visit variant's discriminants
        if let Some((_, ref mut expr)) = node.discriminant {
            self.visit_expr_mut(expr);
        }
    }

    fn visit_fields_mut(&mut self, node: &mut Fields) {
        match node {
            Fields::Named(fields) => {
                for field in fields.named.iter_mut() {
                    self.visit_field_mut(field);
                }
            }
            Fields::Unnamed(fields) => {
                for field in fields.unnamed.iter_mut() {
                    self.visit_field_mut(field);
                }
            }
            _ => {}
        }
    }

    fn visit_field_mut(&mut self, node: &mut Field) {
        // replace field's identifier if it has one
        self.replace_possible_identifier(node);

        // visit the field's type
        self.visit_type_mut(&mut node.ty);
    }

    fn visit_member_mut(&mut self, node: &mut Member) {
        // replace member's identifier if it has one
        self.replace_possible_identifier(node);
    }

    fn visit_pat_mut(&mut self, node: &mut Pat) {
        use Pat::*;
        match node {
            Ident(pat_ident) => self.visit_pat_ident_mut(pat_ident),
            Tuple(pat_tuple) => self.visit_pat_tuple_mut(pat_tuple),
            TupleStruct(pat_tuple_struct) => self.visit_pat_tuple_struct_mut(pat_tuple_struct),
            Type(pat_type) => self.visit_pat_type_mut(pat_type),
            _ => {}
        }
    }

    fn visit_pat_type_mut(&mut self, node: &mut PatType) {
        self.visit_pat_mut(&mut node.pat);
    }

    fn visit_type_mut(&mut self, node: &mut Type) {
        // only replace the type's identifier if it already has a
        // pre-existing mapping, i.e. it's a user-defined type
        if let Type::Path(type_path) = node {
            let mut segments = type_path.path.segments.clone();

            for segment in segments.iter_mut() {
                self.replace_identifier_if_mapped(segment);
            }

            type_path.path.segments = segments;
        }
    }

    fn visit_fn_arg_mut(&mut self, node: &mut FnArg) {
        if let FnArg::Typed(pat_type) = node {
            self.visit_pat_type_mut(pat_type);
        }
    }

    fn visit_expr_mut(&mut self, node: &mut Expr) {
        use Expr::*;
        match node {
            Assign(expr_assign_op) => self.visit_expr_assign_mut(expr_assign_op),
            Binary(expr_binary) => self.visit_expr_binary_mut(expr_binary),
            Block(expr_block) => self.visit_expr_block_mut(expr_block),
            Call(expr_call) => self.visit_expr_call_mut(expr_call),
            Closure(expr_closure) => self.visit_expr_closure_mut(expr_closure),
            Field(expr_field) => self.visit_expr_field_mut(expr_field),
            ForLoop(expr_for_loop) => self.visit_expr_for_loop_mut(expr_for_loop),
            If(expr_if) => self.visit_expr_if_mut(expr_if),
            Let(expr_let) => self.visit_expr_let_mut(expr_let),
            Loop(expr_loop) => self.visit_expr_loop_mut(expr_loop),
            Macro(expr_macro) => self.visit_macro_mut(&mut expr_macro.mac),
            Match(expr_match) => self.visit_expr_match_mut(expr_match),
            MethodCall(expr_method_call) => self.visit_expr_method_call_mut(expr_method_call),
            Path(expr_path) => self.visit_expr_path_mut(expr_path),
            Unary(expr_unary) => self.visit_expr_unary_mut(expr_unary),
            While(expr_while) => self.visit_expr_while_mut(expr_while),
            _ => {}
        }
    }

    fn visit_expr_assign_mut(&mut self, node: &mut ExprAssign) {
        // visit the left and right sides of the assignment op
        self.visit_expr_mut(&mut node.left);
        self.visit_expr_mut(&mut node.right);
    }

    fn visit_expr_binary_mut(&mut self, node: &mut ExprBinary) {
        // visit the left and right sides of the binary op
        self.visit_expr_mut(&mut node.left);
        self.visit_expr_mut(&mut node.right);
    }

    fn visit_expr_match_mut(&mut self, node: &mut ExprMatch) {
        // visit the match's expression
        self.visit_expr_mut(&mut node.expr);

        // visit the match's arms
        for arm in node.arms.iter_mut() {
            self.visit_arm_mut(arm);
        }
    }

    fn visit_expr_method_call_mut(&mut self, node: &mut ExprMethodCall) {
        // visit method call's receiver
        self.visit_expr_mut(&mut node.receiver);

        // replace method identifier only if it was defined by the user
        self.replace_identifier_if_mapped(node);

        // visit method call's arguments
        for arg in node.args.iter_mut() {
            self.visit_expr_mut(arg);
        }
    }

    fn visit_expr_path_mut(&mut self, node: &mut ExprPath) {
        self.visit_path_mut(&mut node.path);
    }

    fn visit_expr_call_mut(&mut self, node: &mut ExprCall) {
        // visit the call's function name
        self.visit_expr_mut(&mut node.func);

        // visit the call's arguments
        for arg in node.args.iter_mut() {
            self.visit_expr_mut(arg);
        }
    }

    fn visit_expr_closure_mut(&mut self, node: &mut ExprClosure) {
        // visit the closure's inputs
        for input in node.inputs.iter_mut() {
            self.visit_pat_mut(input);
        }

        // visit the closure's body
        self.visit_expr_mut(&mut node.body);
    }

    fn visit_expr_field_mut(&mut self, node: &mut ExprField) {
        // visit the struct field's base
        self.visit_expr_mut(&mut node.base);

        // visit the struct field's member
        self.visit_member_mut(&mut node.member);
    }

    fn visit_expr_for_loop_mut(&mut self, node: &mut ExprForLoop) {
        // visit the for loop's pattern
        self.visit_pat_mut(&mut node.pat);

        // visit the for loop's expression
        self.visit_expr_mut(&mut node.expr);

        // visit the for loop's body
        self.visit_block_mut(&mut node.body);
    }

    fn visit_expr_loop_mut(&mut self, node: &mut ExprLoop) {
        // visit the loop's body
        self.visit_block_mut(&mut node.body);
    }

    fn visit_expr_while_mut(&mut self, node: &mut ExprWhile) {
        // visit while loop's condition
        self.visit_expr_mut(&mut node.cond);

        // visit while loop's body
        self.visit_block_mut(&mut node.body);
    }

    fn visit_expr_if_mut(&mut self, node: &mut ExprIf) {
        // visit if expression's condition
        self.visit_expr_mut(&mut node.cond);

        // visit if expression's then branch
        self.visit_block_mut(&mut node.then_branch);

        // visit if expression's else branches
        if let Some((_, ref mut expr)) = node.else_branch {
            self.visit_expr_mut(expr);
        }
    }

    fn visit_expr_unary_mut(&mut self, node: &mut ExprUnary) {
        // visit the unary's expression
        self.visit_expr_mut(&mut node.expr);
    }

    fn visit_expr_let_mut(&mut self, node: &mut ExprLet) {
        // visit let guard's pattern
        self.visit_pat_mut(&mut node.pat);

        // visit let guard's expression
        self.visit_expr_mut(&mut node.expr);
    }

    fn visit_path_mut(&mut self, node: &mut Path) {
        // visit a path if it is an identifier
        if node.get_ident().is_some() {
            // this is fine since we know `node` only contains a
            // single PathSegment which is an identifier
            self.visit_path_segment_mut(node.segments.first_mut().unwrap());
        }
    }

    fn visit_path_segment_mut(&mut self, node: &mut PathSegment) {
        self.replace_identifier(node);
    }

    fn visit_pat_tuple_mut(&mut self, node: &mut PatTuple) {
        // visit the elements of the tuple
        for elem in node.elems.iter_mut() {
            self.visit_pat_mut(elem);
        }
    }

    fn visit_arm_mut(&mut self, node: &mut Arm) {
        // visit arm's expression
        self.visit_pat_mut(&mut node.pat);

        // visit arm's guard if it exists
        if let Some((_, expr)) = &mut node.guard {
            self.visit_expr_mut(expr);
        }

        // visit arm's body
        self.visit_expr_mut(&mut node.body);
    }

    fn visit_macro_mut(&mut self, node: &mut Macro) {
        // parser to parse the body of the macro
        let parser = Punctuated::<Expr, Token![,]>::parse_terminated;

        if let Ok(ref mut body) = node.parse_body_with(parser) {
            // visit each expression of the parsed macro body
            for expr in body.iter_mut() {
                self.visit_expr_mut(expr);
            }

            // update the node with the transformed body
            node.tokens = quote!(#body);
        }
    }
}

use proc_macro2::{Ident, Span};
use syn::{
    ExprMethodCall, Field, ItemConst, ItemEnum, ItemStatic, ItemStruct, ItemType, ItemUnion,
    Member, PatIdent, PathSegment, Signature, Variant,
};

// encapsulates fetching and updating a node's identifier
pub trait ReplaceIdentifier {
    fn ident_string(&self) -> String;
    fn set_ident(&mut self, ident: String);
}

// encapsulates fetching and updating a node that might
// have an identifier
pub trait ReplaceIdentifierMaybe {
    fn ident_string(&self) -> Option<String>;
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

impl ReplaceIdentifier for Variant {
    fn ident_string(&self) -> String {
        self.ident.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.ident = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifier for ExprMethodCall {
    fn ident_string(&self) -> String {
        self.method.to_string()
    }

    fn set_ident(&mut self, ident: String) {
        self.method = Ident::new(&ident, Span::call_site());
    }
}

impl ReplaceIdentifierMaybe for Field {
    fn ident_string(&self) -> Option<String> {
        self.ident.as_ref().map_or(None, |i| Some(i.to_string()))
    }

    fn set_ident(&mut self, ident: String) {
        if let Some(_) = self.ident {
            self.ident = Some(Ident::new(&ident, Span::call_site()));
        }
    }
}

impl ReplaceIdentifierMaybe for Member {
    fn ident_string(&self) -> Option<String> {
        if let Member::Named(ident) = self {
            Some(ident.to_string())
        } else {
            None
        }
    }

    fn set_ident(&mut self, ident: String) {
        if let Member::Named(i) = self {
            *i = Ident::new(&ident, Span::call_site());
        }
    }
}

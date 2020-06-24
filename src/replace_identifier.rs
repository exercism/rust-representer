use proc_macro2::{Ident, Span};
use syn::{
    ItemConst, ItemEnum, ItemStatic, ItemStruct, ItemType, ItemUnion, PatIdent, PathSegment,
    Signature,
};

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

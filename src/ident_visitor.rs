use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::replace_identifier::*;

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

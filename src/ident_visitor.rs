use crate::replace_identifier::*;
use crate::PLACEHOLDER;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

// Set to persist any keywords for identifiers
// we don't want to be replaced
lazy_static! {
    static ref KEYWORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("main");
        s.insert("Some");
        s.insert("None");
        s.insert("self");
        s
    };
}

#[derive(Debug)]
pub struct IdentVisitor {
    // holds each replaced identifier with its placeholder ID
    pub mappings: HashMap<String, u32>,
    // counter to generate the next placeholder ID
    uid: u32,
}

impl IdentVisitor {
    pub fn new() -> Self {
        IdentVisitor {
            mappings: HashMap::new(),
            uid: 0,
        }
    }

    // generates a new placeholder string if the identifier hasn't been seen
    // before, or fetches the pre-existing placeholder if it has been seen before
    fn get_or_insert_mapping(&mut self, ident: String) -> String {
        let uid = match self.mappings.entry(ident) {
            Entry::Occupied(o) => o.into_mut(),
            Entry::Vacant(v) => {
                self.uid += 1;
                v.insert(self.uid)
            }
        };

        format!("{}{}", PLACEHOLDER, uid)
    }

    // fetches the pre-existing placeholder if it has been seen before
    fn get_mapping(&mut self, ident: String) -> Option<String> {
        if let Entry::Occupied(o) = self.mappings.entry(ident) {
            Some(format!("{}{}", PLACEHOLDER, o.into_mut()))
        } else {
            None
        }
    }

    // updates the node's identifier, replacing it with a placeholder
    pub fn replace_identifier<Node: ReplaceIdentifier>(&mut self, node: &mut Node) {
        let ident_string = node.ident_string();

        if !KEYWORDS.contains::<str>(&ident_string) {
            let identifier = self.get_or_insert_mapping(ident_string);

            node.set_ident(identifier);
        }
    }

    // updates the node's identifier if the identifier has already been mapped
    pub fn replace_identifier_if_mapped<Node: ReplaceIdentifier>(&mut self, node: &mut Node) {
        let ident_string = node.ident_string();

        if !KEYWORDS.contains::<str>(&ident_string) {
            if let Some(identifier) = self.get_mapping(ident_string) {
                node.set_ident(identifier);
            }
        }
    }

    // updates the node's identifier if it has one, replacing it with a placeholder
    pub fn replace_possible_identifier<Node: ReplacePossibleIdentifier>(
        &mut self,
        node: &mut Node,
    ) {
        if let Some(ident_string) = node.ident_string() {
            if !KEYWORDS.contains::<str>(&ident_string) {
                let identifier = self.get_or_insert_mapping(ident_string);

                node.set_ident(identifier);
            }
        }
    }
}

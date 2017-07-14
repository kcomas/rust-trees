
use super::node::{MabeNode, IsNode, Node};
use super::tree_serialize::TreeSerialize;
use serde_json;

pub struct Container {
    pub root: MabeNode,
}

impl Container {
    pub fn new(value: u32) -> Container {
        let root = Node::new(None, value);
        Container { root: Some(root) }
    }

    #[allow(dead_code)]
    pub fn add_child(&self, node: IsNode) {
        if let Some(ref r) = self.root {
            r.borrow_mut().add_child(&node, r);
        }
    }

    pub fn add_value(&self, value: u32) {
        if let Some(ref r) = self.root {
            r.borrow_mut().add_value(value, r);
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if let Some(ref r) = self.root {
            r.borrow().print();
        }
    }

    pub fn print_tree(&self) {
        if let Some(ref r) = self.root {
            r.borrow().print_tree();
        }
    }

    pub fn to_json(&self, pretty: bool) -> String {
        if let Some(ref r) = self.root {
            let tree = TreeSerialize::new(r);
            if pretty {
                return serde_json::to_string_pretty(&tree).unwrap();
            } else {
                return serde_json::to_string(&tree).unwrap();
            }
        }
        String::from("{}")
    }
}

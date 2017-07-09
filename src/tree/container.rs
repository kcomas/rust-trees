
use super::node::{MabeNode, IsNode, Node};

pub struct Container {
    root: MabeNode,
}

impl Container {
    pub fn new(value: u32) -> Container {
        let root = Node::new(None, value);
        Container { root: Some(root) }
    }

    pub fn add_child(&self, node: IsNode) {
        if let Some(ref r) = self.root {
            r.borrow_mut().add_child(&node, r);
        }
    }

    pub fn print(&self) {
        if let Some(ref r) = self.root {
            r.borrow().print();
        }
    }
}

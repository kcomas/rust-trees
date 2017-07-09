
use super::node::{MabeNode, IsNode, Node};

pub struct Container {
    root: MabeNode,
}

impl Container {

    pub fn new(value: u32) -> Container {
        let root = Node::new(None, value);
        Container {
            root: Some(root)
        }
    }


}

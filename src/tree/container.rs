
use super::node::{IsNode, Node};

struct Container {
    root: IsNode,
}

impl Container {

    pub fn new(value: u32) -> Container {
        let root = Node::new(None, value);
        Container {
            root: Some(root)
        }
    }


}

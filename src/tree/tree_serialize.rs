
extern crate serde;
extern crate serde_json;

use super::node::{IsNode, MabeNode};

#[derive(Serialize, Deserialize)]
pub struct TreeSerialize {
    name: String,
    children: Vec<TreeSerialize>,
}

impl TreeSerialize {
    pub fn new(node: IsNode) -> TreeSerialize {
        let n = node.borrow();

        let tree_base = TreeSerialize {
            name: n.value.to_string(),
            children: Vec::new(),
        };

        tree_base
    }
}

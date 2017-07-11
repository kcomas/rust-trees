
use super::node::{IsNode, RefNode};

#[derive(Serialize, Deserialize)]
pub struct TreeSerialize {
    name: String,
    children: Vec<TreeSerialize>,
}

impl TreeSerialize {
    pub fn new(node: &IsNode) -> TreeSerialize {
        let n = node.borrow();

        let mut tree_base = TreeSerialize {
            name: n.value.to_string(),
            children: Vec::new(),
        };

        recurse_load(&n, &mut tree_base);

        tree_base
    }

    pub fn get_last_child(&mut self) -> &mut TreeSerialize {
        return self.children.last_mut().unwrap();
    }
}

fn recurse_load(parent_node: &RefNode, parent_json: &mut TreeSerialize) {

    let mut insert = |node: &IsNode| {
        let n = node.borrow();
        let tree = TreeSerialize {
            name: n.value.to_string(),
            children: Vec::new(),
        };
        parent_json.children.push(tree);
        recurse_load(&n, parent_json.get_last_child());
    };

    if let Some(ref left) = parent_node.left {
        insert(left);
    }

    if let Some(ref right) = parent_node.right {
        insert(right);
    }
}

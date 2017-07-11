
use std::cell::{RefCell, RefMut, Ref};
use std::rc::Rc;

pub type IsNode = Rc<RefCell<Node>>;
pub type MabeNode = Option<IsNode>;
pub type RefMutNode<'a> = RefMut<'a, Node>;
pub type RefNode<'a> = Ref<'a, Node>;

pub struct Node {
    pub parent: MabeNode,
    pub left: MabeNode,
    pub right: MabeNode,
    pub value: u32,
}

enum NodeType {
    Left,
    Right,
    Root,
}

impl Node {
    pub fn new(parent: MabeNode, value: u32) -> IsNode {
        Rc::new(RefCell::new(Node {
            parent: parent,
            left: None,
            right: None,
            value: value,
        }))
    }

    #[allow(dead_code)]
    pub fn insert_child(&mut self, child: IsNode) {
        let c = child.borrow();
        if c.value < self.value {
            self.left = Some(child.clone());
        } else if c.value > self.value {
            self.right = Some(child.clone());
        }
    }

    fn link_node(
        &mut self,
        child: &IsNode,
        c: &mut RefMut<Node>,
        parent: &IsNode,
        n_type: NodeType,
    ) {
        let new_child = Some(child.clone());
        c.parent = Some(parent.clone());
        match n_type {
            NodeType::Left => {
                self.left = new_child;
            }
            NodeType::Right => {
                self.right = new_child;
            }
            _ => (),
        }
    }


    fn add_mutable_child(&mut self, child: &IsNode, c: &mut RefMutNode, parent: &IsNode) {
        if c.value < self.value {
            if let Some(ref left) = self.left {
                if let Ok(mut l) = left.try_borrow_mut() {
                    l.add_mutable_child(child, c, left);
                }
            } else {
                self.link_node(child, c, parent, NodeType::Left);
            }
        } else if c.value > self.value {
            if let Some(ref right) = self.right {
                if let Ok(mut r) = right.try_borrow_mut() {
                    r.add_mutable_child(child, c, right);
                }
            } else {
                self.link_node(child, c, parent, NodeType::Right);
            }
        }
    }

    pub fn add_child(&mut self, child: &IsNode, parent: &IsNode) {
        let mut c = child.borrow_mut();
        self.add_mutable_child(child, &mut c, parent);
    }

    pub fn add_value(&mut self, value: u32, parent: &IsNode) {
        let new_node = Node::new(None, value);
        self.add_child(&new_node, parent);
    }

    pub fn print(&self) {
        println!("-----");
        if let Some(ref p) = self.parent {
            println!("Parent: {}", p.borrow().value);
        }
        println!("Value: {}", self.value);
        if let Some(ref c) = self.left {
            println!("Left: {}", c.borrow().value);
        }
        if let Some(ref c) = self.right {
            println!("Right: {}", c.borrow().value);
        }
        println!("-----");
    }

    pub fn print_tree(&self) {
        recurse_print(self, 0, NodeType::Root);
    }
}

fn recurse_print(node: &Node, level_counter: usize, n_type: NodeType) {

    let mut x = 0;
    while x < (level_counter * 2) {
        print!("-");
        x += 1;
    }

    if let Some(ref parent) = node.parent {
        print!("{}", parent.borrow().value);
    } else {
        print!("X");
    }

    match n_type {
        NodeType::Left => print!("L"),
        NodeType::Right => print!("R"),
        NodeType::Root => print!("^"),
    }

    print!("-:{}", node.value);
    println!("");

    if let Some(ref c) = node.left {
        let left = c.borrow();
        recurse_print(&*left, level_counter + 1, NodeType::Left);
    }

    if let Some(ref c) = node.right {
        let right = c.borrow();
        recurse_print(&*right, level_counter + 1, NodeType::Right);
    }

}

#[cfg(test)]
mod test {

    use super::{Node, RefMutNode};

    fn test_parent(node: &RefMutNode, test_val: u32) {
        assert!(node.parent.is_some() == true);
        if let Some(ref p) = node.parent {
            assert!(p.borrow().value == test_val);
        }
    }

    #[test]
    fn basic() {
        let root = Node::new(None, 10);
        let left = Node::new(Some(root.clone()), 5);
        let right = Node::new(Some(root.clone()), 15);

        {
            let mut r = root.borrow_mut();
            assert!(r.value == 10);
            assert!(r.parent.is_none() == true);
            r.insert_child(left.clone());
            r.insert_child(right.clone());
        }

        let mut l = left.borrow_mut();
        assert!(l.value == 5);
        test_parent(&l, 10);
        let sub_left = Node::new(Some(left.clone()), 1);
        let sub_right = Node::new(Some(left.clone()), 9);
        l.insert_child(sub_left.clone());
        l.insert_child(sub_right.clone());

        let mut ri = right.borrow_mut();
        assert!(ri.value == 15);
        test_parent(&ri, 10);
        let sub_left = Node::new(Some(right.clone()), 11);
        let sub_right = Node::new(Some(right.clone()), 20);
        ri.insert_child(sub_left.clone());
        ri.insert_child(sub_right.clone());
    }
}

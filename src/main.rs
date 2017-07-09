
mod tree;
use tree::node::Node;

fn main() {
    let root = Node::new(None, 10);
    let left = Node::new(Some(root.clone()), 5);
    let right = Node::new(Some(root.clone()), 15);
    {
        let mut r = root.borrow_mut();
        r.add_child(left.clone());
        r.add_child(right.clone());
        r.print();
    }
    {
        let mut l = left.borrow_mut();
        let sub_left = Node::new(Some(left.clone()), 1);
        let sub_right = Node::new(Some(left.clone()), 9);
        l.add_child(sub_left.clone());
        l.add_child(sub_right.clone());
        l.print();
    }
    {
        let mut r = right.borrow_mut();
        let sub_left = Node::new(Some(right.clone()), 11);
        let sub_right = Node::new(Some(right.clone()), 20);
        r.add_child(sub_left.clone());
        r.add_child(sub_right.clone());
        r.print();
    }
}

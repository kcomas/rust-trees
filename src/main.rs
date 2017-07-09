
mod tree;
use tree::node::Node;
use tree::printer::Printer;

fn print_test(name: &str) {
    println!("------ {} ------", name);
}

fn basic_tests() {
    print_test("Basic");
    let root = Node::new(None, 10);
    let left = Node::new(Some(root.clone()), 5);
    let right = Node::new(Some(root.clone()), 15);
    {
        let mut r = root.borrow_mut();
        r.insert_child(left.clone());
        r.insert_child(right.clone());
        r.print();
    }
    {
        let mut l = left.borrow_mut();
        let sub_left = Node::new(Some(left.clone()), 1);
        let sub_right = Node::new(Some(left.clone()), 9);
        l.insert_child(sub_left.clone());
        l.insert_child(sub_right.clone());
        l.print();
    }
    {
        let mut r = right.borrow_mut();
        let sub_left = Node::new(Some(right.clone()), 11);
        let sub_right = Node::new(Some(right.clone()), 20);
        r.insert_child(sub_left.clone());
        r.insert_child(sub_right.clone());
        r.print();
    }
    {
        let printer = Printer::new(root.clone());
        printer.basic_print();
    }
    {
        let r = root.borrow();
        r.print_tree();
    }
    {
        let l = left.borrow();
        l.print_tree();
    }
    print_test("Basic");
}

fn container_test() {
    print_test("Container");
}

fn main() {
    basic_tests();
    container_test();
}

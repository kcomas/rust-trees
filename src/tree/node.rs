
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    pub left: Option<Rc<RefCell<Node>>>,
    pub right: Option<Rc<RefCell<Node>>>,
    pub value: u32,
}

impl Node {
    pub fn new(parent: Option<Rc<RefCell<Node>>>, value: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            parent: parent,
            left: None,
            right: None,
            value: value,
        }))
    }

    pub fn add_child(&mut self, child: Rc<RefCell<Node>>) {
        let c = child.borrow();
        if c.value < self.value {
            self.left = Some(child.clone());
        } else if c.value > self.value {
            self.right = Some(child.clone());
        }
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
        recurse_print(self, 0);
    }
}

fn recurse_print(node: &Node, level_counter: usize) {

    let mut x = 0;
    while x < (level_counter * 2) {
        print!("-");
        x += 1;
    }

    print!("-:{}", node.value);
    println!("");

    if let Some(ref c) = node.left {
        let left = c.borrow();
        recurse_print(&*left, level_counter + 1);
    }

    if let Some(ref c) = node.right {
        let right = c.borrow();
        recurse_print(&*right, level_counter + 1);
    }

}

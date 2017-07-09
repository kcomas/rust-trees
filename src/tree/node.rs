
use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    parent: Option<Rc<RefCell<Node>>>,
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    value: i32,
}

impl Node {
    pub fn new(parent: Option<Rc<RefCell<Node>>>, value: i32) -> Rc<RefCell<Node>> {
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

    pub fn print_graph(&self) {

    }
}

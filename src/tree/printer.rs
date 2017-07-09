
use std::cell::RefCell;
use std::rc::Rc;
use super::node::Node;

struct Item {
    value: i32
}

struct Level {
    items: Vec<Item>,
}

pub struct Printer {
    levels: Vec<Level>,
}

impl Printer {
    pub fn new(root: Rc<RefCell<Node>>) -> Printer {
        let mut print = Printer { levels: Vec::new() };

        let level_counter = 0;

        traverse(&mut print, &root, level_counter);

        print
    }

    pub fn basic_print(&self) {
        for (i, level) in self.levels.iter().enumerate() {
            println!("Level: {}", i);
            for (subi, item) in level.items.iter().enumerate() {
                println!("Position: {}, Value: {}", subi, item.value);
            }
        }
    }
}

fn traverse(print: &mut Printer, node: &Rc<RefCell<Node>>, level_counter: usize) {
    let n = node.borrow();
    let item = Item { value: n.value };

    if print.levels.len() > level_counter {
        print.levels[level_counter].items.push(item);
    } else {
        let mut level = Level { items: Vec::new() };
        level.items.push(item);
        print.levels.push(level);
    }

    if let Some(ref child) = n.left {
        traverse(print, child, level_counter + 1);
    }
    if let Some(ref child) = n.right {
        traverse(print, child, level_counter + 1);
    }
}

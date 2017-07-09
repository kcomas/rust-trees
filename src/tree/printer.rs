
use super::node::{IsNode};

struct Item {
    value: i64,
}

struct Level {
    items: Vec<Item>,
}

pub struct Printer {
    levels: Vec<Level>,
}

impl Printer {
    pub fn new(root: IsNode) -> Printer {
        let mut print = Printer { levels: Vec::new() };

        let level_counter = 0;

        traverse(&mut print, &root, level_counter);

        print
    }

    fn largest(&self) -> (usize, usize) {
        let longest_line = self.levels[self.levels.len() - 1].items.len();
        let mut largest_number = 0;
        for level in self.levels.iter() {
            for item in level.items.iter() {
                if item.value > largest_number {
                    largest_number = item.value;
                }
            }
        }
        (longest_line, number_magnatude(largest_number as f64))
    }

    pub fn basic_print(&self) {
        for (i, level) in self.levels.iter().enumerate() {
            println!("Level: {}", i);
            for (subi, item) in level.items.iter().enumerate() {
                println!("Position: {}, Value: {}", subi, item.value);
            }
        }
        let (longest, largest) = self.largest();
        println!("Longest: {}, Largest: {}", longest, largest);
    }
}


fn number_magnatude(mut float: f64) -> usize {
    float = float.log(10.0);
    float = float.floor();
    float += 1.0;
    float as usize
}

fn insert(print: &mut Printer, value: i64, level_counter: usize) {
    let item = Item { value: value };

    if print.levels.len() > level_counter {
        print.levels[level_counter].items.push(item);
    } else {
        let mut level = Level { items: Vec::new() };
        level.items.push(item);
        print.levels.push(level);
    }
}

fn traverse(print: &mut Printer, node: &IsNode, level_counter: usize) {
    let n = node.borrow();

    insert(print, n.value as i64, level_counter);

    if let Some(ref child) = n.left {
        traverse(print, child, level_counter + 1);
    } else {
        insert(print, -1, level_counter + 1);
    }
    if let Some(ref child) = n.right {
        traverse(print, child, level_counter + 1);
    } else {
        insert(print, -1, level_counter + 1);
    }
}

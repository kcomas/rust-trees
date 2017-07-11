
mod tree;
use tree::container::Container;

extern crate rand;
use rand::Rng;

#[macro_use]
extern crate serde_derive;

fn container_test() {
    let cont = Container::new(50);
    #[allow(unused_variables)]
    for x in 0..10 {
        let num: u32 = rand::thread_rng().gen_range(0, 100);
        cont.add_value(num);
    }
    cont.print_tree();
}

fn main() {
    container_test();
}

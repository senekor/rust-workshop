#![allow(unused)]

use std::{borrow::Borrow, cell::RefCell, collections::btree_map, rc::Rc};

struct PrintWhenDropped(char);

impl Drop for PrintWhenDropped {
    fn drop(&mut self) {
        println!("drop called on {}!", self.0)
    }
}

struct GraphNode {
    value: PrintWhenDropped,
    neighbor: Option<Rc<RefCell<GraphNode>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(GraphNode {
        value: PrintWhenDropped('a'),
        neighbor: None,
    }));
    let b = Rc::new(RefCell::new(GraphNode {
        value: PrintWhenDropped('b'),
        neighbor: Some(Rc::clone(&a)),
    }));

    // This line creates a memory leak!
    // - Fix it with weak references (`Rc::downgrade`).
    // - show usage of weak reference
    //
    // a.borrow_mut().neighbor.replace(Rc::clone(&b));

    let temp = Rc::borrow(&a);
    let temp = RefCell::borrow(temp);
    match &temp.neighbor {
        Some(neighbor) => {
            let temp = Rc::borrow(neighbor);
            let temp = RefCell::borrow(temp);
            println!("the neighbor of a is {}", temp.value.0);
        }
        None => println!("a has no neighbor"),
    }
}

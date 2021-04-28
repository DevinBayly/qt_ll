use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum El<T> {
    None,
    Some(Rc<RefCell<Box<T>>>),
}

impl<T> El<T> {
    fn new_part(data: T) -> Rc<RefCell<Box<T>>> {
        Rc::new(RefCell::new(Box::new(data)))
    }
}
#[derive(Debug)]
struct Tree {
    val: f32,
}

#[derive(Debug)]
struct TwoTree {
    val: f32,
    ne: El<TwoTree>,
    nw: El<TwoTree>,
    se: El<TwoTree>,
    sw: El<TwoTree>,
}

impl TwoTree {
    fn new(v: f32) -> Self {
        TwoTree {
            val: v,
            ne: El::None,
            nw: El::None,
            se: El::None,
            sw: El::None,
        }
    }
}

fn main() {
    println!("Hello, world!");
    let head = El::new_part(TwoTree::new(0.0));
    // make a copy of head via rc clone,
    let mut walker = Rc::clone(&head);
    // then loop and add to it
    for i in 0..20 {
        // make a new Element then add it
        let tree_part = TwoTree::new(i as f32);
        let new_ele = El::new_part(tree_part);
        let next_walker = Rc::clone(&new_ele);
        // replace the none at the end of walker
        walker.borrow_mut().ne = El::Some(new_ele);
        // make walker = new_ele
        walker = next_walker;
    }
    println!("result {:?}", head);
}

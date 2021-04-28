use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug)]
enum El<T> {
    None,
    Some(Rc<RefCell<Box<(T,El<T>)>>>)
}

impl<T> El<T> {
    fn new_part(data:T) -> Rc<RefCell<Box<(T,El<T>)>>> {
        Rc::new(RefCell::new(Box::new((data,El::<T>::None))))
    }
}
#[derive(Debug)]
struct Tree {
    val:f32,
}

#[derive(Debug)]
struct TwoTree {
    val:f32,
    left:El::<TwoTree>,
    right:El::<TwoTree>,
}


fn main() {
    println!("Hello, world!");
    let head = El::new_part(Tree{
        val:0.0
    });
    // make a copy of head via rc clone,
    let mut walker = Rc::clone(&head);
    // then loop and add to it
    for i in 0..20 {
        // make a new Element then add it
        let new_ele = Rc::new(RefCell::new(Box::new(
            (
            Tree {
                val:i as f32,
            },El::<Tree>::None
            )  
        )));
        let next_walker = Rc::clone(&new_ele);
        // replace the none at the end of walker
        walker.borrow_mut().1 = El::<Tree>::Some(new_ele);
        // make walker = new_ele
        walker = next_walker;
    }
    println!("result {:#?}",head);

}

use std::cell::RefCell;
use std::rc::{Weak, Rc};

// Rc 共享
// RefCell 可修改
// Weak 弱引用
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node{
        value: 3,
        parent: RefCell::new(Default::default()),
        children: RefCell::new(vec![])
    });

    println!("{:?}", leaf.parent.borrow().upgrade());
    println!("{}", Rc::strong_count(&leaf));
    println!("{}", Rc::weak_count(&leaf));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Default::default()),
        children: RefCell::new(vec![Rc::clone(&leaf), Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("{:?}", leaf.parent.borrow().upgrade());
    println!("{}", Rc::strong_count(&branch));
    println!("{}", Rc::weak_count(&branch));

}

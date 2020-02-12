use std::ops::{Deref, DerefMut};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let add = |x, y| x + y;

    println!("{}", add(3, 5));

    let x = String::from("hello world");
    let print = move || println!("{}", x);
    //println!("{}", x);
    print();

    let b = vec![2, 3];
    for i in b.iter() {
        println!("{}", i);
    }

    let t = Test::new();
    for i in t.take(3) {
        println!("{}", i)
    }

    let x = 5;
    let y = &x;
    let z = Box::new(x);       // 分配到堆上 copy?
    assert_eq!(x, *y);
    assert_eq!(x, *z);
    assert_eq!(*y, *z);

    let i = 6;
    let j = &i;

    let k = MyBox::new( i);
    assert_eq!(i, *k);
    assert_eq!(*j, *k);

    {
        let a = TestDrop{};
        drop(a);
        println!("123123123");

    }
    println!("xxxxx");

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", b);
    println!("{:?}", c);
}


struct Test {
    a: u32
}

impl Test {
    fn new() -> Self {
        Test{a: 0}
    }
}

impl Iterator for Test {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.a += 1;
        if self.a < 6 {
            Some(self.a)
        } else {
            None
        }
    }

}

struct MyBox<T>(T);

impl<T> MyBox<T> {

    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

struct TestDrop();

impl Drop for TestDrop {
    fn drop(&mut self) {
        println!("drop")
    }
}


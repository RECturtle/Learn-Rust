use std::ops::Deref;
use crate::List::{Cons, Nil};

fn main() {
    // Storing data in Box<T> (this is stored on the Heap)
    let b = Box::new(5);
    println!("b = {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // references are a type of pointer
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // * is a dereference operator -> tells computer follow reference
    // to find what y is pointing at. In this case x, which = 5.
    assert_eq!(5, *y);

    // Here's an example using a Box. Only difference is it's a box pointing
    // to value of c VS. reference pointing to value of x in the previous ex.
    let c = 5;
    let d = Box::new(c);
    assert_eq!(5, c);
    assert_eq!(5, *d);

    // Calling my box
    let e = 5;
    let f = MyBox::new(e);
    assert_eq!(5, e);
    assert_eq!(5, *f);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
} // b is deallocated - Box utilize the same scoping rules

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Building a smart pointer as an example
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

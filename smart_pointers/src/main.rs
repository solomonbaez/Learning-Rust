
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }   
}

use std::ops::Deref;
// automaticaly interpreted by the compiler
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CSP with data '{}'!", self.data);
    }
}

use std::mem::drop;
use std::rc::Rc;
use std::cell::RefCell;
use crate::List::{Cons, Nil};

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // equivalent code: hello(&(*m)[..]);

    let c = CustomSmartPointer {
        data: String::from("Rust"),
    };

    let d = CustomSmartPointer {
        data: String::from("Ferris"),
    };
    println!("CSP created.");
    drop(c);
    drop(d);
    println!("CSP(s) dropped prematurely.");


    let val = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // unwrap and modify RefCell
    *val.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

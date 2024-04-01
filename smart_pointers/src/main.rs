/*
// fn main() {
//     let b = Box::new(5); // save 5 on heap 
//     println!("b = {}", b);
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // assert passed so they are equal (with auto deref)


    let x = 5;
    let y = MyBox::new(x); // also passed so Box could work like native ref 

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let s = Box::new(String::from("rust"));
    hello(&s); // or hello(&(*m)[..]); 
    // explain the auto convert done by compiler with deref traits 
}

// 评论：这不是unique_ptr吗？？？ 

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// impl the MyBox<T> by taking its ownership 

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    } // give compiler how to deref it could deref 
    // when *y, rust actually run *(y.deref()) 
}

// impl Deref traits for MyBox pointer 

// also rust have derefmut trait 
// rust will coerce a mutable ref o immutable one 
// from &T -> &U (when T by Deref<Target=U>) 
// from &mut T -> &mut U (when T by DerefMut<Target=U>) 
// from &mut T -> &U (when T: Deref<Target=U>)

fn hello(name: &str) {
    println!("Hello, {name}!");
}
*/

/*

struct CustomSmartPointer {
    data: String,
}

// rust impl drop traits for free allocated data 
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
// results: 
// CustomSmartPointers created.
// Dropping CustomSmartPointer with data `other stuff`!
// Dropping CustomSmartPointer with data `my stuff`!


// you cannot call drop manually because rust will always
// try drop all the data when it goes out of scope which caused
// double free error.

// std::mem:drop is a function that you can call manually
// to drop the data manually.

// Rc<T>: yet another shared_ptr<T> 
// count ref whenever clone is called
// strong count by deafult 
// also we had weak count as Weak<T> 

*/

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // also examples from **the book** 
    // it shows we can change elements in inmutable lists with RefCell<T> 
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
} 

// YSU 的大物实验预习需要手抄实验报告册，快跑！ 
// 所以今天没有进度（04/01/2024） 
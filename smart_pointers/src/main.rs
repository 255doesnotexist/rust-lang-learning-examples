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
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
    let y = Box::new(x); // also passed so Box could work like native ref 

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

// 评论：这不是unique_ptr吗？？？ 
// fn main() {
//     let b = Box::new(5); // save 5 on heap 
//     println!("b = {}", b);
// }

enum List {
    Cons(i32, List),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
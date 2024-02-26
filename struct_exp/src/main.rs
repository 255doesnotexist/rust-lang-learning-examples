#[derive(Debug)]
struct User {
    name: String,
    age: i32,
}

impl User {
    // self could be ref or mut ref or borrow or mut borrow 
    // depends on your needs 
    fn greeting(&self) {
        println!("nice to meet you {}.", self.name);
    }
}

fn main() {
    println!("Hello, world!");

    let u1 = User {
        name: String::from("user1"),
        age: 19
    };

    println!("u1 name: {} age: {}", u1.name, u1.age);
    // apply Debug traits (as we derived Debug before) 
    println!("{:?}\n{:#?}", u1, u1);
    u1.greeting();
}

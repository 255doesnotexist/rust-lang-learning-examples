struct User {
    name: String,
    age: i32,
}

fn main() {
    println!("Hello, world!");

    let u1 = User {
        name: String::from("user1"),
        age: 19
    };

    println!("u1 name: {} age: {}", u1.name, u1.age);
}

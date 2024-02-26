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

    fn older(&self, user2: &User) -> bool {
        self.age > user2.age
    }

    // association function (usually constructor) 
    fn create(name: String, age: i32) -> User {
        User {
            name, // field have same name with the variable so 可以简写 
            age,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let u1 = User {
        name: String::from("user1"),
        age: 19,
    };

    println!("u1 name: {} age: {}", u1.name, u1.age);
    // apply Debug traits (as we derived Debug before) 
    println!("{:?}\n{:#?}", u1, u1);
    u1.greeting();

    let u2 = User {
        name: String::from("user2"),
        age: 18,
    };

    println!("{} is {} than {}", u1.name, if u1.older(&u2) {"older"} else {"younger"}, u2.name);

    let u3 = User::create("user3".to_string(), 21);
    println!("{:?}", u3);
}

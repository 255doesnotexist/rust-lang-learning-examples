fn five() -> i32 {
    5
}

fn main() {
    println!("Hello, world!");

    let x = 5 + 5 * 6;
    let x = {
        let x = x * 10;
        x + 1
    }; // the last var will be returned. 

    println!("var x = {}", x);
    // (5 + 5 * 6) * 10 + 1 = 351

    println!("five function returns = {}", five());
    // the last thing before the brackets will be returned xixi 
}
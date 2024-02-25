fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    println!("The value of x is {}", x);

    let x = x + 1; // shadowing
    println!("Varible x now = {}", x);
}

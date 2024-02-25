fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    println!("The value of x is {}", x);

    let x = x + 1; // shadowing
    println!("Varible x now = {}", x);

    let x = x.to_string(); // shadowed x could be another type 
    println!("x:String = {}", x);

    // i/u 8 16 32 64 128 f 32 64 
}

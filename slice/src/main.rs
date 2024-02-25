fn main() {
    println!("Hello, world!");
    
    let s1 = "unsliced";
    let s2 = &s1[2..]; // [left, right) -> left..right
    println!("{} {}", s1, s2);
    // unsliced sliced

    let a1 = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for (i, ai) in a1[3..6].iter().enumerate() {
        println!("a1[{}] = {}", i, ai);
    }
    // 3, 4, 5

    // slice won't take ownership
}

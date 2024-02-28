fn main() {
    println!("Hello, world!");

    let s1 = "s1".to_string();
    let s2 = "s2".to_string();
    let s3 = "s3".to_string();

    let s4 = s1 + &s2 + &s3;
    // s4 take s1's ownership and just borrow s2 and s3 
    println!("{}", s4);
}

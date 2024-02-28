fn main() {
    println!("Hello, world!");

    let s1 = "s1".to_string();
    let s2 = "s2".to_string();
    let s3 = "s3".to_string();

    // let s4 = s1 + &s2 + &s3;
    // // s4 take s1's ownership and just borrow s2 and s3 
    // println!("{}", s4);

    let s5 = s1.clone() + &s2 + &s3;
    println!("{}", s5);
    // if you make a clone of s1 it will just copy and take nothing 

    let s6 = format!("{}{}{}", s1, s2, s3);
    // or why dont you try format!? 
    println!("{}", s6);

    // 3 ways to see string: s.bytes()：字节流, s.chars()：Unicode 标量值, 标准库中没有：字形簇 
}

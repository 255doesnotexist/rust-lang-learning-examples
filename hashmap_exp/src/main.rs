use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    let mut hmap: HashMap<String, i32> = HashMap::new();

    // 实现 copy trait 的会被复制 实现所有权的会被移动进去 
    hmap.insert(String::from("one"), 1);
    let q1 = hmap.get(&"one".to_string());
    let q2 = hmap.get(&"two".to_string());

    match q1 {
        Some(num) => {println!("{} -> {}", "one", num);}
        None => {println!("{} -> None", "one");}
    }

    match q2 {
        Some(num) => {println!("{} -> {}", "two", num);}
        None => {println!("{} -> None", "two");}
    }
}

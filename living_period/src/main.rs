fn main() {
    println!("Hello, world!");

    let a = "A".to_string();
    let c;
    {
        let b = "B".to_string();
        c = longest(&a, &b);
    }
    println!("{}", c);
}

fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
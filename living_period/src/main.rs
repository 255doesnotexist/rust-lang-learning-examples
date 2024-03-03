fn main() {
    println!("Hello, world!");

    // let a = "A".to_string();
    // let c;
    // {
    //     let b = "B".to_string();
    //     c = longest(&a, &b);
    // }
    // println!("{}", c);
    // borrowed value does not live long enough

    let a = "A".to_string();
    let b = "B".to_string();
    let c;
    {
        c = longest(&a, &b);
    } // that's right 
    println!("{}", c);
}

fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
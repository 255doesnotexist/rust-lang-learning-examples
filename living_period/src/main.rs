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

    let ts = TwoStrings {
        str1: &String::from("first"),
        str2: &String::from("second"),
    };
}

fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

struct TwoStrings <'a> {
    str1: &'a String,
    str2: &'a String,
}
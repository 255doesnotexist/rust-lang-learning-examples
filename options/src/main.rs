fn main() {
    println!("Hello, world!");

    // option is an type template that artificially support null (None) 

    let n1: Option<i32> = None;
    let n2: Option<i32> = Some(0623);

    println!("n1 = {}, n2 = {}", n1.unwrap_or(-1), n2.unwrap());
}

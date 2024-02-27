fn main() {
    println!("Hello, world!");

    let v = vec![1, 2, 3, 4];
    println!("v[2] = {}", v[2]);

    match v.get(2) {
        Some(num) => {println!("v.get(2) = Some({})", num);}
        None => {println!("out of index");}
    }
}

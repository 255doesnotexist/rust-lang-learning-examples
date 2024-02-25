fn main() {
    println!("Hello, world!");

    let x1 = 1;
    let x2 = x1; // basic type have Copy traits so x1 x2 are ok to print 
    println!("x1 {} x2 {}", x1, x2);

    let s1: String = String::from("string");
    let s2 = s1;
    // println!("{} {}", s1, s2); cannot compile because s1's ownership is moved to s2 
    println!("{}", s2);

    let s1 = s2.clone(); // make a clone (deepcopy) then ownership is not a problem
    println!("cloned {} s2 {}", s1, s2);

    let _t = takeit(s2);
    // println!("{}", s2); cannot compile because byval parameter make s2 moved 

    let mut s3 = String::from("s3");
    let mut s4 = String::from("s4");

    println!("s3 = {}, s4 = {}", s3, s4);
    swap(&mut s3, &mut s4);
    println!("swapped s3 = {}, s4 = {}", s3, s4);
}
fn takeit(str: String) -> usize{
    str.len()
}
fn swap(a: &mut String, b: &mut String) {
    let t = a.clone();
    *a = b.clone(); *b = t;
}
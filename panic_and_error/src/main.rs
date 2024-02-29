use std::io::stdin;

fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");
    // make panic = abort could let rust abort calling stack immediately instead of unwinding the stack 

    // most error doesnt need to panic so we use Result<T, E> to handle it 
    /*
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */

    println!("we are going to have a pretty safe integer dividing calculation.");

    println!("input a: ");
    let mut ts = String::new();
    let a: i32 = {stdin().read_line(&mut ts).expect("failed to read_line"); ts}.trim().parse().expect("not an integer");

    println!("input b: ");
    let mut ts = String::new();
    let b: i32 = {stdin().read_line(&mut ts).expect("failed to read_line"); ts}.trim().parse().expect("not an integer");

    match div(&a, &b) {
        Ok(ans) => {println!("{} / {} = {}", a, b, ans);}
        Err(err) => {println!("{} / {} failed because {}", a, b, err);},
    }
    
}

fn div(a: &i32, b: &i32) -> Result<i32, String> {
    if *b != 0 {
        Ok(a / b)
    }else{
        Err("cannot divide zero".to_string())
    }
}

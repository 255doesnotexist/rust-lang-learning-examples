fn main() {
    println!("Hello, world!");
}

#[test]
fn test() {
    println!("test function acted");

    assert_eq!(1+1, 2); // passed test 
    // assert_eq!(1+1, 3); // this will make test failed 
    panic!("this will make test failed too");
}
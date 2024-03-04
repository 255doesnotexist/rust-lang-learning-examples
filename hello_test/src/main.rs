fn main() {
    println!("Hello, world!");
}

#[test]
#[should_panic] // in this case panic mean test passed
fn test() {
    panic!("now panic");
    println!("test function acted");

    assert_eq!(1+1, 2); // passed test 
    // assert_eq!(1+1, 3); // this will make test failed 
    // assert_ne!(1+1, 2); // this will make test failed 
    // panic!("this will make test failed too");
    // assert!(true or false); // false make test failed

    // assert-family macros could add one more parameter to output customized message  
}
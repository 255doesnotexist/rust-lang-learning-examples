fn main() {
    println!("Hello, world!");
    panic!("crash and burn");
    // make panic = abort could let rust abort calling stack immediately instead of unwinding the stack 
}

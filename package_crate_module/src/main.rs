// mod lib; // import external module lib.rs
// lib is the library crate root of the rust project so donnot mod it or will cause clash  
use package_crate_module::fancyprinting::ezra; // use ezra module in lib crate

// all rust mod/struct are private by default except the enum 
// you could see same level and father level whether private or public 
// you could only see public things in child level 
fn main() {
    println!("Hello, world!");
    // super:: could be ../ 
    // crate:: could be this crate's root 
    ezra::write();
}

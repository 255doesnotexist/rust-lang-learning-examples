use std::{cmp::Ordering, io}; // prelude 是一组自动导入到每个模块作用域中的名称，例如标准库、外部库、语言和工具的 prelude1。它可以简化代码的编写，避免频繁地使用 use 声明或绝对路径。 
use rand::Rng; // trait 是一组为未知类型定义的方法，例如 Self。它可以用来抽象地定义一些共享的行为，也可以用 trait bound 来约束泛型类型必须具有某些行为。trait 可以包含关联函数、常量和类型参数，也可以提供默认的方法实现或被其他 trait 继承。 
fn main() {
    println!("guessing a number");

    let secret_num = rand::thread_rng().gen_range(1..101);
    // generate a random number (注意新版本 rust 需要使用 .. 表示范围而非逗号分隔)
    println!("(*secretly*) number {} generated", secret_num);

    loop{
        println!("guess a number:");

        let mut str = String::new();
        // mut mutable 可变的 默认不可变 
    
        io::stdin().read_line(&mut str)
        .expect("read line failed");
        // io::Result -> Ok, Err
        
        let num: i32 = match str.trim().parse::<i32>(){
            // expect return a Result and have Ok and Err enums
            Ok(num) => num,
            Err(_) => {println!("guess invalid"); continue;}, // invalid so let user retype again
        };
        // remember: trim the new line at the end of the string
    
        println!("the number you guess is {}", num);
    
        // convert (parse) string to int32 first then compare 
        match num.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Equal => {println!("bingo!"); break;},
            Ordering::Greater => println!("too big")
        } // elegant state switch. pretty like it. 
    }
}

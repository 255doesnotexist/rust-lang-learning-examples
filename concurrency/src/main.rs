/* 
fn main() {
    println!("Hello, concurrency world!");
}
*/

/* use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
} */

// The spawned thread will run concurrently with the main thread.
// THe printing line are wholely sperated from each other. It seems atomic? 

// output: 
/* hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread! */

// After the end of the main thread, the spawned thread will be terminated.

/* use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    // this will wait for the spawned thread to finish 
} */

// chatgpt tell me println macro is not atomic so upper related comment is not true 

use rayon::prelude::*;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 创建一个线程池并在其中并行处理向量中的元素
    let sum: i32 = numbers.par_iter().map(|&x| x * x).sum();

    println!("Sum of squares: {}", sum);
}

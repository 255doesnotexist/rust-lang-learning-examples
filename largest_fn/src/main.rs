use std::cmp::max;

fn main() {
    println!("Hello, world!");

    let nums = [1, 3, 5, 7, 9, -1];
    println!("the largest num in list is {}", largest(&nums));
}

// input ref of i32 sliced list and return the max value 
fn largest(list: &[i32]) -> i32 {
    let mut ret: i32 = i32::MIN;
    for &i in list { // &i will auto deref so *i* will be i32. if you put "for i in list" here *i* will be i32 
        ret = max(ret, i);
    }
    ret
}
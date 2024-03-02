use std::cmp::max;

fn main() {
    println!("Hello, world!");

    let nums: [i64; 3] = [1, 2, 3];
    println!("a generic-based largest function = {}", largest(&nums));
}

trait HasMin {
    const MIN: Self;
}
impl HasMin for i64 {
    const MIN: i64 = i64::MIN;
}
impl HasMin for i32 {
    const MIN: i32 = i32::MIN;
}

// fn largest<T: HasMin + std::cmp::Ord + Copy>(list: &[T]) -> T {
fn largest<T>(list: &[T]) -> T 
    where T: HasMin + std::cmp::Ord + Copy {
    let mut ret: T = T::MIN;
    for &i in list { // &i will auto deref so *i* will be i32. if you put "for i in list" here *i* will be i32 
        ret = max(ret, i);
    }
    ret
}

struct Point<T, U> {
    x: T,
    y: U,
}

enum Result<T,E> {
    Ok(T),
    Err(E),
}
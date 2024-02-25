fn main() {
    println!("Hello, world!");

    let mut cnt = 1;

    loop{
        if cnt >= 3 {
            break;
        }
        println!("{}", cnt);
        cnt += 1
    }
    println!("cnt {}", cnt);

    let loop_ret = loop{
        cnt -= 1;
        break cnt;
    };
    println!("loopret {}", loop_ret);

    let while_ret = while cnt >= -3 {
        cnt -= 4;
    };
    // println!("{}", while_ret); while always return a () so cannot be formatted & printed

    let arr: [i32; 6] = [233, 244, 255, 266, 277, 288];
    for i in arr.iter() {
        println!("arr {}", i);
    }

    for i in (1..10).rev() { // 10..1 will be [] because left range should always < right range .rev() make it reversed 
        println!("you can use for with range {}", i)
    }
}

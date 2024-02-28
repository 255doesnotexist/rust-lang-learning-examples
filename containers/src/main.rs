fn main() {
    println!("Hello, world!");

    let v = vec![1, 2, 3, 4];
    println!("v[2] = {}", v[2]);

    match v.get(2) {
        Some(num) => {println!("v.get(2) = Some({})", num);}
        None => {println!("out of index");}
    }

    let mut vm = vec!["chii".to_string(),"kawaii".to_string()];
    vm.push("cinamoroll".to_string());

    let mut index = 0;
    for s in &mut vm {
        println!("vm[{}] = {}", index, s);
        *s = s.to_owned() + " " + &index.to_string();
        index = index + 1;
    }

    println!("{:#?}", vm);
}

/*
    let mut v = vec![1, 2, 3];
    let v0 = &v[0];
    v.push(4);
    println!("{}", v0);
    // this code wont compile because an immutable ref exists
*/
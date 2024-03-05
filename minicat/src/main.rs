use std::env;
use std::fs::read_to_string;
use std::process::exit;
use std::path::Path;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut contents: Vec<String> = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        let path = Path::new(&arg);
        let content = match read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                eprintln!("Reading the file {} failed.", i, arg);
                exit(1);
            }
        };
        contents.push(content);
    }

    for (i, content) in contents.iter().enumerate() {
        print!("{}", content);
    }

    Ok(())
}

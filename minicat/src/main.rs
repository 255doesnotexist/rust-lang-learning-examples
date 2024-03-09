use std::env;
use std::process;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = minicat::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problems happened because: {}", err);
        process::exit(1); // lambda in rust 
    });

    if let Err(err) = minicat::run(&config) {
        eprintln!("Application problems: {}", err);
        process::exit(1);
    }

    Ok(())
}
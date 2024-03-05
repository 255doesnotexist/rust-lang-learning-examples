use std::env;
use std::fs::read_to_string;
use std::process;
use std::path::Path;
use std::io;
use std::error::Error;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problems happened because: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(&config) {
        eprintln!("Application problems: {}", err);
        process::exit(1);
    }

    Ok(())
}

struct Config {
    contents: Vec<String>
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, Box<dyn Error> > {
        let mut contents: Vec<String> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            let path = Path::new(&arg);
            let content = read_to_string(path)?;
            contents.push(content);
        }
        Ok(Config{ contents })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error> > {
    for (i, content) in config.contents.iter().enumerate() {
        print!("{}", content);
    }
    Ok(())
}
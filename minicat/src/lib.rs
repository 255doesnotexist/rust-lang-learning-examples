use std::fs::read_to_string;
use std::path::Path;
use std::error::Error;

pub struct Config {
    contents: Vec<String>
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, Box<dyn Error> > {
        let mut contents: Vec<String> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            let path = Path::new(&arg);
            let content = read_to_string(path)?;
            contents.push(content);
        }
        Ok(Config{ contents })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error> > {
    for (i, content) in config.contents.iter().enumerate() {
        print!("{}", content);
    }
    Ok(())
}
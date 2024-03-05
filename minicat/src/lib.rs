use std::fs::read_to_string;
use std::path::Path;
use std::process;
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
    print!("{}", concat(&config.contents).unwrap_or_else(|err|{
        eprintln!("Problems when concating: {}", err);
        process::exit(1);
    }));

    Ok(())
}

pub fn concat(v: &Vec<String>) -> Result<String, Box<dyn Error> > {
    let mut ret: String = String::new();

    for (i, content) in v.iter().enumerate() {
        ret += content;
    }

    Ok(ret)
}

# [cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrong_path() {
        let cfg = match Config::new(&vec!["\\totally_wrong_path/wontLocateAnyFile".to_string()]) {
            Ok(s) => Err(s),
            Err(e) => Ok(e),
        };
    }

    #[test]
    fn concat_test() -> Result<(), Box<dyn Error> > {
        let result = concat(&vec!["first".to_string(), "second".to_string()]).unwrap();

        assert_eq!(result, "firstsecond");
        Ok(())
    }
}
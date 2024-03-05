use std::fs::read_to_string;
use std::path::Path;
use std::process;
use std::error::Error;
use std::env;

pub struct Config {
    pub contents: Vec<String>,
    pub append_with_crlf: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, Box<dyn Error> > {
        let mut contents: Vec<String> = Vec::new();
        for (i, arg) in args.iter().enumerate() {
            let path = Path::new(&arg);
            let content = read_to_string(path)?;
            contents.push(content);
        }

        let append_with_crlf = env::var("CONCAT_APPEND_WITH_CRLF").is_err();
        Ok(Config{ contents, append_with_crlf })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error> > {
    if config.append_with_crlf {
        print!("{}", concat_with_crlf(&config.contents).unwrap_or_else(|err|{
            eprintln!("Problems when concating: {}", err);
            process::exit(1);
        }));
    } else {
        print!("{}", concat(&config.contents).unwrap_or_else(|err|{
            eprintln!("Problems when concating: {}", err);
            process::exit(1);
        }));
    }

    Ok(())
}

pub fn concat(v: &Vec<String>) -> Result<String, Box<dyn Error> > {
    let mut ret: String = String::new();

    for (i, content) in v.iter().enumerate() {
        ret += content;
    }

    Ok(ret)
}

pub fn concat_with_crlf(v: &Vec<String>) -> Result<String, Box<dyn Error> > {
    let mut ret: String = String::new();

    for (i, content) in v.iter().enumerate() {
        ret += content;
        if i != v.len() - 1 {
            ret += "\n";
        }
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
        let result_with_crlf = concat_with_crlf(&vec!["first".to_string(), "second".to_string()]).unwrap();

        assert_eq!(result, "firstsecond");
        assert_eq!(result_with_crlf, "first\nsecond");
        Ok(())
    }
}
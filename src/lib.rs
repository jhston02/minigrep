use std::error::Error;
use std::fs;

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(config.filename)?;

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use crate::Config;
    use super::*;

    #[test]
    fn instantiate_config_1_value_errors() {
        let args = vec!(String::from("Howdy"));
        if let Err(e) = Config::new(&args) {
            assert_eq!(e, "not enough arguments");
        }
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
        Rust:
        safe, fast, productive
        Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, content));
    }
}
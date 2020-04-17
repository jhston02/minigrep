use std::error::Error;
use std::fs;

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("Searching in {}", config.filename);

    let content = fs::read_to_string(config.filename)?;
    println!("{}", content);

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
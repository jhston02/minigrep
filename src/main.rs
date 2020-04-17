use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("Searching in {}", config.filename);

    let content = fs::read_to_string(config.filename).expect("Could not read file");
    println!("{}", content);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguemts");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

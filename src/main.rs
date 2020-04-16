use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_configs(&args);

    println!("Searching for {}", query);
    println!("Searching in {}", filename);

    let content = fs::read_to_string(filename).expect("Could not read file");
    println!("{}", content);
}

fn parse_configs(args: &[String]) -> (&String, &String) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In fle {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config<'a>(args: &'a [&str]) -> Config<'a> {
    let query = args[1];
    let filename = args[2];

    Config { query, filename }
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = Config::new(&args);

    let query = &args.query;
    let filename = &args.filename;

    println!("Searching for {}", query);
    println!("In fle {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Config<'a> {
        if args.len() < 3 {
            panic!("not enough arguments");
        }

        Config {
            query: &args[1],
            filename: &args[2],
        }
    }
}

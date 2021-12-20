use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.filename);

    if let Err(e) = run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl<'a> Config<'a> {
    fn new(args: &'a [String]) -> Result<Config<'a>, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

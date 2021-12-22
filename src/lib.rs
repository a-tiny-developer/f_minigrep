use std::{error::Error, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
}

impl Config<'_> {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: &args[1],
            filename: &args[2],
        })
    }
}
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe,fast,productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

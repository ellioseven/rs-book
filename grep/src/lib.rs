use std::{fs};
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = search(&config.query, &contents);

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments.")
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let config = Config { query, filename };

        Ok(config)
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "a";
        let contents = "foo\nbar\nbaz";
        assert_eq!(vec!["bar", "baz"], search(query, contents));
    }
}
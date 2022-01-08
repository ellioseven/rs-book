use std::{env, fs};
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() != 3 {
            return Err("not enough arguments.")
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        // `is_err` checks `CASE_INSENSITIVE` is not set.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let config = Config { query, filename, case_sensitive };

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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "a";
        let contents = "foo\nbar\nbaz";
        assert_eq!(vec!["bar", "baz"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "foo";
        let contents = "Foo\nBar\nBaz";
        assert_eq!(vec!["Foo"], search_case_insensitive(query, contents));
    }
}
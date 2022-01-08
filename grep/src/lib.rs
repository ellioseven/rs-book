use std::{fs};
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    println!("{}", contents);
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

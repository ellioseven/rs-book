use std::{env, process};
use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("config error: {}", e);
        process::exit(1);
    });

    if let Err(e) = grep::run(&config) {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

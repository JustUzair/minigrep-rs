use std::env;
use std::process;

use minigrep_rs::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        print!("⚠️ Error parsing arguments: {}\n", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        print!("⚠️ Application error: {}\n", e);
        process::exit(1);
    }
}

use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching for '{}' in file '{}'",
        config.query, config.filename
    );

    let contents = fs::read_to_string(config.filename)?;

    // println!("\n\n📃 File contents:\n{}\n\n", contents);

    for (line_num, line) in search(&config.query, &contents) {
        println!("Line {}: {}", line_num, line);
    }

    Ok(())
}

pub fn search<'i>(query: &str, contents: &'i str) -> Vec<(u32, &'i str)> {
    // let mut results: Vec<_> = Vec::new();
    // for (line_num, line) in contents.lines().enumerate() {
    //     if line.to_lowercase().contains(query.to_lowercase().as_str()) {
    //         results.push(((line_num + 1) as u32, line));
    //     }
    // }
    // results
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .map(|(line_num, line)| ((line_num + 1) as u32, line))
        .collect()
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        // let filename = args.next().unwrap();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = r"\ 
Rust:
safe, fast, productive.
Pick three. Duct tape.";

        assert_eq!(
            vec![
                (3, "safe, fast, productive."),
                (4, "Pick three. Duct tape.")
            ],
            search(query, contents)
        );
    }
    #[test]
    fn test_search_case_insensitive() {
        let query = "DuCT";
        let contents = r"\ 
Rust:
safe, fast, productive.
Pick three. Duct tape.";

        assert_eq!(
            vec![
                (3, "safe, fast, productive."),
                (4, "Pick three. Duct tape.")
            ],
            search(query, contents)
        );
    }
}

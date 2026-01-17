use std::error::Error;
use std::fs;

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
    let mut results: Vec<_> = Vec::new();
    for (line_num, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(query.to_lowercase().as_str()) {
            results.push(((line_num + 1) as u32, line));
        }
    }
    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

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

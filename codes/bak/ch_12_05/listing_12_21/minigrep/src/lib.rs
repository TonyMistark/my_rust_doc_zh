use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}


impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contains = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contains) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a> (query: &str, contains: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contains.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a> (
    query: &str,
    contains: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contains.lines() {
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
        let query = "duct";
        let contains = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contains));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contains = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contains));
    }
}


use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments -> expected 3");
        }
        // somewhat inneficient but deep copy the data
        let query = args[1].clone();
        let file_path = args[2].clone();
        // set case sensitivity
        let ignore_case = if env::var("IGNORE_CASE").is_ok() {
            println!("Case insensitive analysis:");
            true
        }   else {
            println!("Case sensitive analysis:");
            false
        };

        Ok(Config {query, file_path, ignore_case})
    }
}

// assign lifetime signature to the contents as we want them returned
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    // search for contents
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    // return contents
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results 
}

// allow for dynamic error report
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }   
        else {
            search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "safe,";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], 
            search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "ruSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents));
    }
}
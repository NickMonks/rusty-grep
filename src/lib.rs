use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    // the "?" will invoke the from function, defined in the From Trait
    let contents = fs::read_to_string(config.filename)?;

    // perform search
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    // If no error, return unit type ()
    // Otherwise return the trait object Error 
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        
        // perform deep copy of args immutable ref
        // so Config has owned values. Not performant
        let query = args[1].clone();
        let filename = args[2].clone();
        
        // If case insensitive exists, is_err will return false (is a Result)
        // otherwise it will give an error and is_err sets to true
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query, 
            filename,
            case_sensitive,
        })
    }
}

// We specify the relationship with input and output through lifetime
// the data returned will live as long as the data passed in contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// --- TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "orld";
        let contents = "\
        hello world, I am very
        happy today";

        assert_eq!(vec!["hello world, I am very"], search(query, contents));
    }

    #[test]
    fn case_insensitive_test() {
        let query = "WorLd";
        let contents = "\
        hello world, I am very
        happy today";

        assert_eq!(vec!["hello world, I am very"], search(query, contents));
    }
}
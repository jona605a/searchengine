use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub indexno: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 4 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let indexno = args[3].clone();

        Ok(Config { query, file_path, indexno })
    }
}

pub struct Index<T> {
    pub indexno: String,
    pub database: T,
}

impl Index<HashMap<String,HashSet<String>>> {
    pub fn build(config: Config) {
        let mut database: HashMap<String,HashSet<String>> = HashMap::new();
        let mut index: Index<HashMap<String,HashSet<String>>> = Index {indexno: config.indexno, database};
        // Do the indexing
        


    }
}



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    // let index: Index<HashMap<String,HashSet<String>>> = Index {
    //     indexno: String::from("6"),
    //     database: HashMap<String,HashSet<String>>::new();
    // } ;
    let index: Index<HashMap<String,HashSet<String>>>  = Index { indexno: String::from("6"), database: HashMap::new()};

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
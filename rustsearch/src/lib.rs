use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;

// mod index;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub indexno: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
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

impl Index<HashMap<&str,HashSet<&str>>> {
    pub fn build<'a>(config: Config) -> Result<Index<HashMap<&'a str,HashSet<&'a str>>>, Box<dyn Error>> {
        
        let mut database: HashMap<&str,HashSet<&str>> = HashMap::new();
        
        // Do the indexing
        let filecontents = read_file_to_string(config.file_path)?;

        let mut prev_word = "---END.OF.DOCUMENT---";
        let mut cur_title = "";
        for word in filecontents.split_whitespace() {
            if word == "---END.OF.DOCUMENT---" {continue}
            // Update title
            if prev_word == "---END.OF.DOCUMENT---" {
                cur_title = word;
            }

            database.entry(word)
                .or_default()
                .insert(cur_title);



            prev_word = word;
        }

        let index: Index<HashMap<&str,HashSet<&str>>> = Index {indexno: config.indexno, database};
        Ok(index)
    }
}

fn read_file_to_string(file_path: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    

    let index: Index<HashMap<String,HashSet<String>>>  = Index { indexno: String::from("6"), database: HashMap::new()};

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

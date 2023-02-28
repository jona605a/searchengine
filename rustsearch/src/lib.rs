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
    indexno: String,
    database: T,
}

impl Index<HashMap<String,HashSet<String>>> {
    pub fn build(config: &Config) -> Result<Index<HashMap<String,HashSet<String>>>, Box<dyn Error>> {
        
        let mut database: HashMap<String,HashSet<String>> = HashMap::new();
        
        let filecontents = read_file_to_string(&config.file_path)?;

        let mut prev_word = String::from("---END.OF.DOCUMENT---");
        let mut cur_title = String::new();
        for word in filecontents.split_whitespace() {
            if word == "---END.OF.DOCUMENT---" {
                prev_word = word.to_string();
                continue;
            }
            // Update title
            if prev_word == "---END.OF.DOCUMENT---" {
                cur_title = word.to_string();
                prev_word = String::new();
            }

            database.entry(word.to_string())
                .or_default()
                .insert(cur_title.clone());
        }

        let index: Index<HashMap<String,HashSet<String>>> = Index {indexno: config.indexno.clone(), database};
        Ok(index)
    }

    pub fn search(&self, word: &String) -> Option<&HashSet<String>> {
        self.database.get(word)
    }
}

fn read_file_to_string(file_path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    

    let index = Index::build(&config)?;
    println!("indexno: {}",index.indexno);

    match index.search(&config.query) {
        Some(result) => {
            println!("{:?}", result)
        }
        None => println!("Word not found"),
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

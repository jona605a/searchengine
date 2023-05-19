use std::collections::HashMap;
use std::error::Error;
use std::fs;

use regex::Regex;

use crate::index::{Index, Search};

pub struct Config {
    pub file_path: String,
    pub indexno: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let file_path = args[1].clone();
        let indexno = args[2].clone();

        Ok(Config { file_path, indexno })
    }

    pub fn to_index(&self) -> Result<Box<dyn Search>, Box<dyn Error>> {
        match self.indexno.as_str() {
            "6" => Ok(Box::new(Index::index6(&self)?)),
            "7" => Ok(Box::new(Index::index7(&self)?)),
            "7_0" => Ok(Box::new(Index::index7(&self)?)),
            "8" => Ok(Box::new(Index::index8(&self)?)),
            "9_0" => Ok(Box::new(Index::index9_0(&self)?)),
            "9_1" => Ok(Box::new(Index::index9_1(&self)?)),
            "10" => Ok(Box::new(Index::index10(&self)?)),
            "10_0" => Ok(Box::new(Index::index10(&self)?)),
            "10_1" => Ok(Box::new(Index::index10(&self)?)),
            "11" => Ok(Box::new(Index::index11(&self)?)),
            "11_0" => Ok(Box::new(Index::index11(&self)?)),
            "11_1" => Ok(Box::new(Index::index11(&self)?)),
            _ => unimplemented!(),
        }
    }
}

pub fn read_file_to_string(file_path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

pub fn read_and_clean_file_to_iter(config: &Config) -> Result<Vec<(String,Vec<String>)>, Box<dyn Error>> {
    let filecontents = fs::read_to_string(&config.file_path)?;
    let re = Regex::new(r"\. |\.\n|\.\r\n|\n\n|; |[\[\]\{\}\\\n\(\) ,:/=?!*]").unwrap();

    // Articles are seperated by the delimiter "---END.OF.DOCUMENT---"
    // In each article, it is assumed that the first line is the title, ending in a '.'
    // The contents of each article is split according to the regular expression.

    let articles_iter: Vec<(String, Vec<String>)> = filecontents.split("---END.OF.DOCUMENT---").map(|a| {
        // let (title, contents) = a.trim().split_once(".\n").unwrap_or(("", ""));
        let (title, contents) = match a.trim().split_once(".\n") {
            Some((t, c)) => (t, c),
            None => a.trim().split_once(".\r\n") // Some Windows shit
                        .unwrap_or(("", "")), 
        };
        let x: Vec<String> = re.split(contents).filter(|&s| s!="").map(|s| s.to_string()).collect();
        (title.to_string(), x)
    }).collect();
    Ok(articles_iter)
}

pub fn word_freq() {
    let file5mb = "data/WestburyLab.wikicorp.201004_5MB.txt";
    let file_contents = fs::read_to_string(&file5mb.to_string()).unwrap();
    // let re = Regex::new(r#"^[[:alpha:]/''`\-]"#).unwrap();
    let re = Regex::new(r#"\. |\.\n|; |[\[\]\{\}\\\n\(\) ",:/=?!*]"#).unwrap();
    let articles_iter = file_contents.split("---END.OF.DOCUMENT---").map(|a| {
        let (title, contents) = a.trim().split_once(".\n").unwrap_or(("", ""));
        (title.to_string(), re.split(contents))
    });
    let mut word_freq: HashMap<String, usize> = HashMap::new();
    for (title, contents) in articles_iter {
        if title != "" {
            for word in contents {
                word_freq
                    .entry(word.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
            }
        }
    }
    println!("{:#?}", word_freq)
}

pub fn write_article_files(config: &Config) {
    let articles_iter = read_and_clean_file_to_iter(&config).unwrap();

    let mut count = 0;

    for (title, contents) in articles_iter {
        if title != "" {
            let x = contents.join(" ");
            fs::write(format!("data/individual_articles/{:05}.txt", count), x).unwrap();
            count += 1;
        }
        if count > 99999 {
            panic!("Too many articles. It's over 99999.")
        }
    }
}


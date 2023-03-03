use std::collections::{HashMap, HashSet};
use std::error::Error;
use regex::Regex;

use crate::index::Index;
use crate::helpers::*;

#[allow(dead_code)]

impl Index<HashMap<String,HashSet<String>>,Option<u128>> {
    pub fn index6(config: &Config) -> Result<Index<HashMap<String,HashSet<String>>,Option<u128>>, Box<dyn Error>> {
        
        let mut database: HashMap<String,HashSet<String>> = HashMap::new();
        
        let filecontents = read_file_to_string(&config.file_path)?;
        let re = Regex::new(r"\. |\.\n|\n\n|[\[\]\{\}\\\n\() ,;:/=?!*&]").unwrap();

        let mut prev_word = String::from("---END.OF.DOCUMENT---");
        let mut cur_title = String::new();

        let mut x = re.split(&filecontents);
        x.next();

        for word in  x{

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

        let index: Index<HashMap<String,HashSet<String>>,Option<u128>> = 
            Index { 
                database, 
                extra_variables: None
            };
        Ok(index)
        
    }

    pub fn search(&self, word: &String) -> Option<&HashSet<String>> {
        self.database.get(word)
    }
}




#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn 
}
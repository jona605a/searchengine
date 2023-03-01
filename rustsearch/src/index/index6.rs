use std::collections::{HashMap, HashSet};
use std::error::Error;

use crate::index::Index;
use crate::helpers::*;

// use rustsearch::

impl Index<HashMap<String,HashSet<String>>> {
    pub fn index6(config: &Config) -> Result<Index<HashMap<String,HashSet<String>>>, Box<dyn Error>> {
        
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

        let index: Index<HashMap<String,HashSet<String>>> = Index {database};
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